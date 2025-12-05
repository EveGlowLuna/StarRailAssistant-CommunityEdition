<template>
    <div class="version-update-window" ref="windowRef">
        <div class="version-update-container">
            <h2 class="window-title">{{ t('home.versionUpdate') }}</h2>
            <p class="placeholder-text">版本更新功能即将实现...</p>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from '@tauri-apps/api/core';
import { useTranslation } from "../composables/useTranslation";

const { t } = useTranslation();

const windowRef = ref<HTMLElement | null>(null);

// 加载壁纸
const loadWallpaper = async () => {
  try {
    const base64Data = await invoke<string | null>('get_wallpaper_base64');
    
    if (base64Data && windowRef.value) {
      windowRef.value.style.setProperty('background-image', `url('${base64Data}')`, 'important');
      windowRef.value.style.setProperty('background-size', 'cover', 'important');
      windowRef.value.style.setProperty('background-position', 'center', 'important');
      windowRef.value.style.setProperty('background-repeat', 'no-repeat', 'important');
      console.log('Wallpaper applied to version update window');
    }
  } catch (error) {
    console.error("Failed to load wallpaper:", error);
  }
};

onMounted(() => {
    loadWallpaper();
});
</script>

<style scoped>
.version-update-window {
  width: 100vw;
  height: 100vh;
  background: url('../assets/background-lt.jpg') no-repeat center center;
  background-size: cover;
  display: flex;
  align-items: center;
  justify-content: center;
}

.version-update-container {
  text-align: center;
  padding: 40px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.window-title {
  margin: 0 0 20px 0;
  font-size: 24px;
  font-weight: 600;
  color: #000;
}

.placeholder-text {
  margin: 0;
  font-size: 16px;
  color: #666;
}

@media (prefers-color-scheme: dark) {
    .version-update-window {
        background: url('../assets/background-lt.jpg') no-repeat center center;
        background-size: cover;
    }

    .version-update-container {
        background: rgba(0, 0, 0, 0.8);
        border: 1px solid rgba(255, 255, 255, 0.2);
    }

    .window-title {
        color: #fff;
    }

    .placeholder-text {
        color: #ccc;
    }
}
</style>
