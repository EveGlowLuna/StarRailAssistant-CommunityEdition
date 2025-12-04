<template>
    <div class="announcement-window">
        <div class="announcement-container">
            <div class="announcement-header">
                <h2 class="announcement-title">{{ t('announcement.title') }}</h2>
                <button
                    class="refresh-button"
                    @click="refreshAnnouncements"
                    :disabled="loading"
                >
                    <RefreshCw :size="16" :class="{ spinning: loading }" />
                    {{ t('announcement.refresh') }}
                </button>
            </div>

            <div class="announcements-layout" v-if="!loading && !error">
                <!-- 左侧公告列表 -->
                <div class="announcements-sidebar">
                    <div class="sidebar-title">{{ t('announcement.title') }}</div>
                    <div class="announcements-menu">
                        <div
                            v-for="(announcement, index) in announcements"
                            :key="index"
                            class="menu-item"
                            :class="{ active: selectedIndex === index }"
                            @click="selectAnnouncement(index)"
                        >
                            <div class="menu-item-content">
                                <span class="menu-item-title">{{ announcement.title }}</span>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- 右侧公告内容 -->
                <div class="announcements-content">
                    <div
                        v-if="announcements.length > 0 && announcements[selectedIndex]"
                        class="announcement-detail"
                    >
                        <div class="announcement-detail-header">
                            <h2 class="announcement-detail-title">
                                {{ announcements[selectedIndex].title }}
                            </h2>
                        </div>
                        <div class="announcement-detail-content">
                            <vue3-markdown-it :source="announcements[selectedIndex].content" />
                        </div>
                    </div>
                    <div v-else class="no-announcement">
                        {{ t('announcement.noAnnouncements') }}
                    </div>
                </div>
            </div>

            <!-- 加载状态 -->
            <div class="loading-state" v-if="loading">
                <RefreshCw class="spinning loading-icon" :size="24" />
                <span class="loading-text">{{ t('announcement.loading') }}</span>
            </div>

            <!-- 错误状态 -->
            <div class="error-state" v-if="error && !loading">
                <AlertCircle :size="24" class="error-icon" />
                <span class="error-text">{{ t('announcement.error') }}</span>
                <button class="retry-button" @click="refreshAnnouncements">
                    {{ t('announcement.retry') }}
                </button>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from '@tauri-apps/api/core';
import Vue3MarkdownIt from "vue3-markdown-it";
import {
    RefreshCw,
    AlertCircle,
} from "lucide-vue-next";
import { useTranslation } from "../composables/useTranslation";

const { t } = useTranslation();

interface Announcement {
    title: string;
    content: string;
}

// 响应式数据
const loading = ref(false);
const error = ref<string | null>(null);
const announcements = ref<Announcement[]>([]);
const expandedCards = ref<Record<number, boolean>>({});
const selectedIndex = ref<number>(0);

// 方法
const loadAnnouncements = async () => {
    loading.value = true;
    error.value = null;

    try {
        // 获取当前语言设置
        const currentLang = localStorage.getItem('language') || 'zh-CN';
        
        const result = await invoke<Announcement[]>('get_announcements', { 
            lang: currentLang 
        });

        announcements.value = result;
        
        // 初始化展开状态，默认展开第一个
        expandedCards.value = {};
        if (result.length > 0) {
            expandedCards.value[0] = true;
            selectedIndex.value = 0;
        }
    } catch (err) {
        const errorMsg = err instanceof Error ? err.message : (typeof err === 'string' ? err : '加载公告失败');
        error.value = errorMsg;
        console.error("Failed to load announcements:", errorMsg, err);
    } finally {
        loading.value = false;
    }
};

const selectAnnouncement = (index: number) => {
    selectedIndex.value = index;
};

const refreshAnnouncements = () => {
    loadAnnouncements();
};

onMounted(() => {
    loadAnnouncements();
});
</script>

<style scoped>
.announcement-window {
  width: 100vw;
  height: 100vh;
  background: url('../assets/background-lt.jpg') no-repeat center center;
  background-size: cover;
  overflow: hidden;
}

.announcement-container {
  height: 100%;
  padding: 16px;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
}



.announcement-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
  padding: 16px 20px;
  flex-shrink: 0;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.announcement-title {
    margin: 0;
    font-size: 24px;
    font-weight: 600;
    color: #000;
}

.refresh-button {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 16px;
    border: none;
    border-radius: 6px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
    background: rgba(0, 0, 0, 0.1);
}

.refresh-button:hover:not(:disabled) {
    background: rgba(0, 0, 0, 0.2);
    transform: translateY(-1px);
}

.refresh-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.spinning {
    animation: spin 1s linear infinite;
}

@keyframes spin {
    from {
        transform: rotate(0deg);
    }
    to {
        transform: rotate(360deg);
    }
}

.slide-fade-enter-active {
    transition: all 0.3s ease;
}

.slide-fade-leave-active {
    transition: all 0.2s ease;
    position: absolute;
    width: 100%;
}

.slide-fade-enter-from {
    opacity: 0;
    transform: translateY(-10px);
}

.slide-fade-enter-to {
    opacity: 1;
    transform: translateY(0);
}

.slide-fade-leave-from {
    opacity: 1;
    transform: translateY(0);
}

.slide-fade-leave-to {
    opacity: 0;
    transform: translateY(-5px);
}

.announcements-layout {
  display: flex;
  gap: 16px;
  flex: 1;
  min-height: 0;
  overflow: hidden;
}

.announcements-sidebar {
  width: 240px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  overflow: hidden;
  display: flex;
  flex-direction: column;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.sidebar-title {
  padding: 16px;
  font-size: 20px;
  font-weight: 600;
  color: #000;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}

.announcements-menu {
  flex: 1;
  overflow-y: auto;
}

.announcements-menu::-webkit-scrollbar {
  width: 6px;
}

.announcements-menu::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.05);
}

.announcements-menu::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 3px;
}

.announcements-menu::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

.menu-item {
  padding: 12px 16px;
  cursor: pointer;
  transition: all 0.2s ease;
  border-left: 3px solid transparent;
  color: rgba(0, 0, 0, 0.7);
}

.menu-item:hover {
  background: rgba(0, 0, 0, 0.05);
  color: rgba(0, 0, 0, 0.9);
}

.menu-item.active {
  background: rgba(0, 0, 0, 0.1);
  border-left-color: #000;
  color: #000;
}

.menu-item-content {
  display: flex;
  align-items: center;
  gap: 8px;
}

.menu-item-title {
  font-size: 14px;
  font-weight: 500;
  line-height: 1.4;
}

.announcements-content {
  flex: 1;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  overflow-y: auto;
  padding: 24px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.announcements-content::-webkit-scrollbar {
  width: 8px;
}

.announcements-content::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.05);
  border-radius: 4px;
}

.announcements-content::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 4px;
}

.announcements-content::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

.announcement-detail-header {
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 2px solid rgba(0, 0, 0, 0.1);
}

.announcement-detail-title {
  margin: 0;
  font-size: 28px;
  font-weight: 700;
  color: #000;
}

.announcement-detail-content {
  color: #000;
  line-height: 1.8;
  font-size: 15px;
}

.no-announcement {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 100%;
  color: #64748b;
  font-size: 16px;
}

.announcement-detail-content :deep(h1),
.announcement-detail-content :deep(h2),
.announcement-detail-content :deep(h3),
.announcement-detail-content :deep(h4),
.announcement-detail-content :deep(h5),
.announcement-detail-content :deep(h6) {
    color: #000;
    margin-top: 1.5em;
    margin-bottom: 0.75em;
    font-weight: 600;
    line-height: 1.3;
}

.announcement-detail-content :deep(h1):first-child,
.announcement-detail-content :deep(h2):first-child,
.announcement-detail-content :deep(h3):first-child {
    margin-top: 0;
}

.announcement-detail-content :deep(h2) {
    font-size: 24px;
}

.announcement-detail-content :deep(h3) {
    font-size: 20px;
}

.announcement-detail-content :deep(p) {
    margin: 1em 0;
    line-height: 1.8;
}

.announcement-detail-content :deep(p):first-child {
    margin-top: 0;
}

.announcement-detail-content :deep(ul),
.announcement-detail-content :deep(ol) {
    margin: 1em 0;
    padding-left: 2em;
}

.announcement-detail-content :deep(li) {
    margin: 0.5em 0;
    line-height: 1.6;
}

.announcement-detail-content :deep(a) {
    color: #3b82f6;
    text-decoration: none;
    transition: color 0.2s ease;
}

.announcement-detail-content :deep(a):hover {
    color: #2563eb;
    text-decoration: underline;
}

.announcement-detail-content :deep(code) {
    background: rgba(0, 0, 0, 0.05);
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 0.9em;
    font-family: 'Consolas', 'Monaco', monospace;
    color: #e11d48;
}

.announcement-detail-content :deep(pre) {
    background: rgba(0, 0, 0, 0.05);
    color: #000;
    padding: 1.25em;
    border-radius: 8px;
    overflow-x: auto;
    margin: 1.5em 0;
}

.announcement-detail-content :deep(pre code) {
    background: none;
    padding: 0;
    color: inherit;
}

.announcement-detail-content :deep(blockquote) {
    border-left: 4px solid rgba(0, 0, 0, 0.3);
    padding-left: 1em;
    margin: 1.5em 0;
    color: rgba(0, 0, 0, 0.7);
    font-style: italic;
}

.announcement-detail-content :deep(hr) {
    border: none;
    height: 1px;
    background: rgba(0, 0, 0, 0.1);
    margin: 2em 0;
}

.loading-state,
.error-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    flex: 1;
    gap: 16px;
    background: rgba(255, 255, 255, 0.8);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 8px;
}

.loading-icon,
.error-icon {
    color: #666;
}

.loading-text,
.error-text {
    font-size: 16px;
    color: #666;
}

.retry-button {
    padding: 8px 16px;
    border: none;
    border-radius: 6px;
    font-size: 14px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.3s ease;
    background: rgba(0, 0, 0, 0.1);
}

.retry-button:hover {
    background: rgba(0, 0, 0, 0.2);
    transform: translateY(-1px);
}

@media (prefers-color-scheme: dark) {
    .announcement-header {
        background: rgba(0, 0, 0, 0.8);
        border: 1px solid rgba(255, 255, 255, 0.2);
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    }

    .announcement-title {
        color: #f1f5f9;
    }

    .refresh-button {
        background: rgba(255, 255, 255, 0.1);
        color: #fff;
    }

    .refresh-button:hover:not(:disabled) {
        background: rgba(255, 255, 255, 0.2);
    }

    .announcements-sidebar {
        background: rgba(0, 0, 0, 0.8);
        border: 1px solid rgba(255, 255, 255, 0.2);
        box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    }

    .sidebar-title {
        color: #fff;
        border-bottom-color: rgba(255, 255, 255, 0.2);
    }

    .menu-item {
        color: rgba(255, 255, 255, 0.7);
    }

    .menu-item:hover {
        background: rgba(255, 255, 255, 0.1);
        color: rgba(255, 255, 255, 0.9);
    }

    .menu-item.active {
        background: rgba(255, 255, 255, 0.2);
        border-left-color: #fff;
        color: #fff;
    }

    .announcements-menu::-webkit-scrollbar-track {
        background: rgba(255, 255, 255, 0.05);
    }

    .announcements-menu::-webkit-scrollbar-thumb {
        background: rgba(255, 255, 255, 0.2);
    }

    .announcements-menu::-webkit-scrollbar-thumb:hover {
        background: rgba(255, 255, 255, 0.3);
    }

    .announcements-content {
        background: rgba(0, 0, 0, 0.8);
        border-color: rgba(255, 255, 255, 0.2);
    }

    .announcement-detail-title {
        color: #f1f5f9;
    }

    .announcement-detail-content {
        color: #cbd5e1;
    }

    .announcement-detail-content :deep(h1),
    .announcement-detail-content :deep(h2),
    .announcement-detail-content :deep(h3),
    .announcement-detail-content :deep(h4),
    .announcement-detail-content :deep(h5),
    .announcement-detail-content :deep(h6) {
        color: #f1f5f9;
    }

    .announcement-detail-content :deep(a) {
        color: #60a5fa;
    }

    .announcement-detail-content :deep(a):hover {
        color: #93c5fd;
    }

    .announcement-detail-content :deep(code) {
        background: rgba(255, 255, 255, 0.1);
        color: #fca5a5;
    }

    .announcement-detail-content :deep(blockquote) {
        border-left-color: #60a5fa;
        color: #94a3b8;
    }

    .announcement-detail-content :deep(hr) {
        background: rgba(255, 255, 255, 0.1);
    }

    .announcement-detail-header {
        border-bottom-color: rgba(255, 255, 255, 0.1);
    }

    .no-announcement {
        color: #94a3b8;
    }

    .loading-state,
    .error-state {
        background: rgba(30, 41, 59, 0.95);
        border-color: rgba(255, 255, 255, 0.1);
    }

    .loading-text,
    .error-text {
        color: #cbd5e1;
    }

    .loading-icon,
    .error-icon {
        color: #94a3b8;
    }

    .refresh-button {
        background: rgba(255, 255, 255, 0.1);
        color: #fff;
    }

    .refresh-button:hover:not(:disabled) {
        background: rgba(255, 255, 255, 0.2);
    }

    .retry-button {
        background: rgba(255, 255, 255, 0.1);
        color: #fff;
    }

    .retry-button:hover {
        background: rgba(255, 255, 255, 0.2);
    }


}
</style>
