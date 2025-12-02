<template>
  <div class="custom-dropdown" :class="{ 'dropdown-open': isOpen }">
    <!-- 下拉框触发器 -->
    <div
      class="dropdown-trigger"
      ref="triggerRef"
      @click="toggleDropdown"
      @blur="handleBlur"
      tabindex="0"
    >
      <span class="selected-value">{{ selectedLabel }}</span>
      <div class="dropdown-arrow" :class="{ 'arrow-up': isOpen }">
        <svg
          width="16"
          height="16"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <polyline points="6 9 12 15 18 9"></polyline>
        </svg>
      </div>
    </div>

    <!-- 使用 Teleport 将菜单移动到 body，避免被父容器裁切 -->
    <Teleport to="body">
      <div
        v-if="isOpen"
        class="dropdown-menu"
        ref="dropdownMenuRef"
        :style="menuStyle"
      >
        <div
          v-for="(option, index) in options"
          :key="index"
          class="dropdown-option"
          :class="{
            'option-selected': modelValue === option.value,
            'option-hover': hoverIndex === index
          }"
          @click="selectOption(option)"
          @mouseenter="hoverIndex = index"
          @mouseleave="hoverIndex = -1"
        >
          {{ option.label }}
        </div>
      </div>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import {
  ref,
  computed,
  onMounted,
  onUnmounted,
  nextTick,
  reactive,
} from "vue";

interface DropdownOption {
  label: string;
  value: string | number;
}

interface Props {
  modelValue: string | number;
  options: DropdownOption[];
  placeholder?: string;
}

interface Emits {
  (e: "update:modelValue", value: string | number): void;
}

const props = withDefaults(defineProps<Props>(), {
  placeholder: "请选择...",
});

const emit = defineEmits<Emits>();

const isOpen = ref(false);
const hoverIndex = ref(-1);

// 下拉菜单定位
const triggerRef = ref<HTMLElement | null>(null);
const dropdownMenuRef = ref<HTMLElement | null>(null);

const menuStyle = reactive({
  position: "fixed" as const,
  top: "0px",
  left: "0px",
  width: "0px",
  zIndex: 999999,
});

// 计算并更新下拉菜单位置
const updateMenuPosition = async () => {
  if (!triggerRef.value) return;

  const rect = triggerRef.value.getBoundingClientRect();
  const maxMenuHeight = 200; // CSS中设置的最大高度
  const spaceBelow = window.innerHeight - rect.bottom;
  const spaceAbove = rect.top;

  // 先设置基本位置，让菜单渲染出来
  menuStyle.left = rect.left + "px";
  menuStyle.width = rect.width + "px";

  // 等待下一帧，获取实际菜单高度
  await nextTick();
  
  if (dropdownMenuRef.value) {
    const actualMenuHeight = dropdownMenuRef.value.offsetHeight;
    
    // 如果下方空间不足且上方空间更多，则向上展开
    if (spaceBelow < actualMenuHeight && spaceAbove > spaceBelow) {
      // 向上展开：菜单底部紧贴触发器顶部
      menuStyle.top = (rect.top - actualMenuHeight) + "px";
    } else {
      // 向下展开：菜单顶部紧贴触发器底部
      menuStyle.top = rect.bottom + "px";
    }
  } else {
    // 如果无法获取实际高度，使用估算
    if (spaceBelow < maxMenuHeight && spaceAbove > spaceBelow) {
      menuStyle.top = (rect.top - Math.min(maxMenuHeight, spaceAbove)) + "px";
    } else {
      menuStyle.top = rect.bottom + "px";
    }
  }
};

// 计算选中的标签
const selectedLabel = computed(() => {
  const selectedOption = props.options.find((option) => {
    // 使用严格比较
    return option.value === props.modelValue;
  });
  return selectedOption ? selectedOption.label : props.placeholder;
});

// 切换下拉框状态
const toggleDropdown = () => {
  isOpen.value = !isOpen.value;

  if (isOpen.value) {
    hoverIndex.value = -1;
    nextTick(() => updateMenuPosition());
  }
};

// 选择选项
const selectOption = (option: DropdownOption) => {
  emit("update:modelValue", option.value);
  isOpen.value = false;
  hoverIndex.value = -1;
};

// 失去焦点，关闭下拉
const handleBlur = (event: FocusEvent) => {
  setTimeout(() => {
    if (!(event.relatedTarget as Element)?.closest(".custom-dropdown")) {
      isOpen.value = false;
      hoverIndex.value = -1;
    }
  }, 150);
};

// 点击外部关闭
const handleClickOutside = (event: MouseEvent) => {
  const target = event.target as Element;
  if (!target.closest(".custom-dropdown")) {
    isOpen.value = false;
    hoverIndex.value = -1;
  }
};

// 键盘导航
const handleKeydown = (event: KeyboardEvent) => {
  if (!isOpen.value) {
    if (event.key === "Enter" || event.key === " ") {
      event.preventDefault();
      toggleDropdown();
    }
    return;
  }

  switch (event.key) {
    case "Escape":
      isOpen.value = false;
      hoverIndex.value = -1;
      break;

    case "ArrowDown":
      event.preventDefault();
      hoverIndex.value = (hoverIndex.value + 1) % props.options.length;
      break;

    case "ArrowUp":
      event.preventDefault();
      hoverIndex.value =
        hoverIndex.value <= 0
          ? props.options.length - 1
          : hoverIndex.value - 1;
      break;

    case "Enter":
      event.preventDefault();
      if (hoverIndex.value >= 0) {
        selectOption(props.options[hoverIndex.value]);
      }
      break;
  }
};

// 生命周期
onMounted(() => {
  document.addEventListener("click", handleClickOutside);
  document.addEventListener("keydown", handleKeydown);
  window.addEventListener("scroll", updateMenuPosition);
  window.addEventListener("resize", updateMenuPosition);
});

onUnmounted(() => {
  document.removeEventListener("click", handleClickOutside);
  document.removeEventListener("keydown", handleKeydown);
  window.removeEventListener("scroll", updateMenuPosition);
  window.removeEventListener("resize", updateMenuPosition);
});
</script>

<style scoped>
.custom-dropdown {
  position: relative;
  width: 100%;
  user-select: none;
}

.dropdown-trigger {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 12px;
  border: 1px solid rgba(0, 0, 0, 0.2);
  border-radius: 4px;
  background: rgba(255, 255, 255, 0.9);
  font-size: 14px;
  cursor: pointer;
  transition: all 0.3s ease;
  min-height: 36px;
  box-sizing: border-box;
}

.dropdown-trigger:focus {
  outline: none;
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.dropdown-trigger:hover {
  border-color: rgba(0, 0, 0, 0.3);
}

.dropdown-open .dropdown-trigger {
  border-color: #007bff;
  box-shadow: 0 0 0 2px rgba(0, 123, 255, 0.25);
}

.selected-value {
  flex: 1;
  color: #000;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.dropdown-arrow {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 16px;
  height: 16px;
  margin-left: 8px;
  transition: transform 0.3s ease;
  color: #666;
  flex-shrink: 0;
}

.arrow-up {
  transform: rotate(180deg);
}

/* Dropdown 菜单（位置由 Teleport + JS 控制） */
.dropdown-menu {
  background: rgba(255, 255, 255, 0.95);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 4px;
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  max-height: 200px;
  overflow-y: auto;
  animation: dropdown-appear 0.2s ease;
}

@keyframes dropdown-appear {
  from {
    opacity: 0;
    transform: translateY(-8px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.dropdown-option {
  padding: 8px 12px;
  font-size: 14px;
  color: #000;
  cursor: pointer;
  transition: all 0.2s ease;
  border-bottom: 1px solid rgba(0, 0, 0, 0.05);
}

.dropdown-option:last-child {
  border-bottom: none;
}

.dropdown-option:hover {
  background: rgba(33, 150, 243, 0.1);
  color: #1976d2;
}

.option-hover {
  background: rgba(33, 150, 243, 0.15);
  color: #1976d2;
}

.option-selected {
  background: rgba(33, 150, 243, 0.8);
  color: white;
}

.option-selected:hover {
  background: rgba(33, 150, 243, 0.9);
  color: white;
}

/* 深色模式 */
@media (prefers-color-scheme: dark) {
  .dropdown-trigger {
    background: rgba(255, 255, 255, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
    color: #fff;
  }

  .selected-value {
    color: #fff;
  }

  .dropdown-arrow {
    color: #ccc;
  }

  .dropdown-menu {
    background: rgba(30, 30, 30, 0.95);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .dropdown-option {
    color: #fff;
    border-bottom: 1px solid rgba(255, 255, 255, 0.1);
  }

  .dropdown-option:hover {
    background: rgba(33, 150, 243, 0.2);
    color: #64b5f6;
  }

  .option-hover {
    background: rgba(33, 150, 243, 0.25);
    color: #64b5f6;
  }

  .option-selected {
    background: rgba(33, 150, 243, 0.8);
    color: white;
  }

  .option-selected:hover {
    background: rgba(33, 150, 243, 0.9);
    color: white;
  }
}

/* 美化滚动条 */
.dropdown-menu::-webkit-scrollbar {
  width: 6px;
}

.dropdown-menu::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.05);
  border-radius: 3px;
}

.dropdown-menu::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 3px;
}

.dropdown-menu::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

@media (prefers-color-scheme: dark) {
  .dropdown-menu::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.05);
  }

  .dropdown-menu::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
  }

  .dropdown-menu::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.3);
  }
}
</style>
