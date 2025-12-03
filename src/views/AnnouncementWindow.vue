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

            <div class="announcements-list" v-if="!loading && !error">
                <!-- 主要公告 -->
                <div
                    class="announcement-card main-announcement"
                    v-if="mainAnnouncement"
                >
                    <div
                        class="announcement-card-header"
                        @click="toggleCard('main')"
                    >
                        <h3 class="announcement-name">{{ t('announcement.mainAnnouncement') }}</h3>
                        <button class="expand-button">
                            <ChevronDown v-if="expandedCards.main" :size="16" />
                            <ChevronUp v-else :size="16" />
                        </button>
                    </div>
                    <transition name="slide-fade">
                        <div
                            class="announcement-content"
                            v-show="expandedCards.main"
                        >
                            <vue3-markdown-it :source="mainAnnouncement.content" />
                        </div>
                    </transition>
                </div>

                <!-- 最新版本公告 -->
                <div
                    class="announcement-card release-announcement"
                    v-if="releaseAnnouncement"
                >
                    <div
                        class="announcement-card-header"
                        @click="toggleCard('release')"
                    >
                        <h3 class="announcement-name">
                            {{ t('announcement.releaseAnnouncement') }}
                        </h3>
                        <button class="expand-button">
                            <ChevronDown v-if="expandedCards.release" :size="16" />
                            <ChevronUp v-else :size="16" />
                        </button>
                    </div>
                    <transition name="slide-fade">
                        <div
                            class="announcement-content"
                            v-show="expandedCards.release"
                        >
                            <vue3-markdown-it
                                :source="releaseAnnouncement.content"
                            />
                        </div>
                    </transition>
                </div>

                <!-- 其他公告 -->
                <div
                    class="announcement-card other-announcements"
                    v-if="otherAnnouncements.length > 0"
                >
                    <div
                        class="announcement-card-header"
                        @click="toggleCard('other')"
                    >
                        <h3 class="announcement-name">{{ t('announcement.otherAnnouncements') }}</h3>
                        <button class="expand-button">
                            <ChevronDown v-if="expandedCards.other" :size="16" />
                            <ChevronUp v-else :size="16" />
                        </button>
                    </div>
                    <transition name="slide-fade">
                        <div
                            class="announcement-content"
                            v-show="expandedCards.other"
                        >
                            <div
                                v-for="announcement in otherAnnouncements"
                                :key="announcement.name"
                                class="other-item"
                            >
                                <vue3-markdown-it :source="announcement.content" />
                                <hr
                                    v-if="
                                        announcement !==
                                        otherAnnouncements[
                                            otherAnnouncements.length - 1
                                        ]
                                    "
                                    class="divider"
                                />
                            </div>
                        </div>
                    </transition>
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
    ChevronDown,
    ChevronUp,
} from "lucide-vue-next";
import { useTranslation } from "../composables/useTranslation";

const { t } = useTranslation();

interface Announcement {
    name: string;
    content: string;
}

// 响应式数据
const loading = ref(false);
const error = ref<string | null>(null);
const mainAnnouncement = ref<Announcement | null>(null);
const releaseAnnouncement = ref<Announcement | null>(null);
const otherAnnouncements = ref<Announcement[]>([]);
const expandedCards = ref({
    main: true,
    release: true,
    other: true,
});

// 方法
const loadAnnouncements = async () => {
    loading.value = true;
    error.value = null;

    try {
        const announcements = await invoke<Announcement[]>('get_announcements');

        mainAnnouncement.value = null;
        releaseAnnouncement.value = null;
        otherAnnouncements.value = [];

        announcements.forEach((announcement) => {
            switch (announcement.name) {
                case "main-announce":
                    mainAnnouncement.value = announcement;
                    break;
                case "release-announce":
                    releaseAnnouncement.value = announcement;
                    break;
                case "announces":
                    otherAnnouncements.value.push(announcement);
                    break;
            }
        });
    } catch (err) {
        const errorMsg = err instanceof Error ? err.message : (typeof err === 'string' ? err : '加载公告失败');
        error.value = errorMsg;
        console.error("Failed to load announcements:", errorMsg, err);
    } finally {
        loading.value = false;
    }
};

const toggleCard = (cardType: "main" | "release" | "other") => {
    expandedCards.value[cardType] = !expandedCards.value[cardType];
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
  background: #f6f6f6;
  overflow: hidden;
}

.announcement-container {
  height: 100%;
  padding: 16px;
  overflow-y: auto;
}

.announcement-container::-webkit-scrollbar {
    width: 8px;
}

.announcement-container::-webkit-scrollbar-track {
    background: rgba(0, 0, 0, 0.05);
    border-radius: 4px;
}

.announcement-container::-webkit-scrollbar-thumb {
    background: rgba(0, 0, 0, 0.2);
    border-radius: 4px;
    transition: background-color 0.3s ease;
}

.announcement-container::-webkit-scrollbar-thumb:hover {
    background: rgba(0, 0, 0, 0.3);
}

.announcement-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 20px;
  padding: 16px 20px;
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

.announcements-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
  padding-bottom: 16px;
}

.announcement-card {
    background: rgba(255, 255, 255, 0.8);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    overflow: hidden;
    margin-bottom: 12px;
    position: relative;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.announcement-card:last-child {
    margin-bottom: 40px;
}

.announcement-card-header {
  padding: 12px 16px;
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
  cursor: pointer;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: background-color 0.3s ease;
  border-radius: 8px;
}

.announcement-card-header:hover {
    background: rgba(255, 255, 255, 0.9);
}

.announcement-name {
    margin: 0;
    font-size: 18px;
    font-weight: 600;
    color: #000;
    flex: 1;
}

.announcement-content {
  padding: 16px;
  color: #000;
  line-height: 1.6;
  text-align: left;
  margin: 0;
}

.announcement-content :deep(h1),
.announcement-content :deep(h2),
.announcement-content :deep(h3),
.announcement-content :deep(h4),
.announcement-content :deep(h5),
.announcement-content :deep(h6) {
    color: #000;
    margin-top: 1.5em;
    margin-bottom: 0.5em;
    text-align: left;
    margin-top: 0;
}

.announcement-content :deep(h1):first-child,
.announcement-content :deep(h2):first-child,
.announcement-content :deep(h3):first-child,
.announcement-content :deep(h4):first-child,
.announcement-content :deep(h5):first-child,
.announcement-content :deep(h6):first-child {
    margin-top: 0;
}

.announcement-content :deep(p) {
    margin: 0.8em 0;
    text-align: left;
}

.announcement-content :deep(p):first-child {
    margin-top: 0;
}

.announcement-content :deep(ul),
.announcement-content :deep(ol) {
    margin: 1em 0;
    padding-left: 2em;
    text-align: left;
}

.announcement-content :deep(ul):first-child,
.announcement-content :deep(ol):first-child {
    margin-top: 0;
}

.announcement-content :deep(li) {
    margin: 0.3em 0;
    text-align: left;
}

.announcement-content :deep(code) {
    background: rgba(0, 0, 0, 0.1);
    padding: 2px 6px;
    border-radius: 3px;
    font-size: 0.9em;
}

.announcement-content :deep(pre) {
    background: rgba(0, 0, 0, 0.05);
    padding: 1em;
    border-radius: 6px;
    overflow-x: auto;
    margin: 1em 0;
    text-align: left;
}

.announcement-content :deep(pre):first-child {
    margin-top: 0;
}

.announcement-content :deep(pre code) {
    background: none;
    padding: 0;
}

.other-item {
    margin-bottom: 1.5em;
    text-align: left;
}

.divider {
    border: none;
    height: 1px;
    background: rgba(0, 0, 0, 0.1);
    margin: 2em 0;
}

.expand-button {
    background: none;
    border: none;
    color: #666;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    transition: all 0.3s ease;
    display: flex;
    align-items: center;
    justify-content: center;
}

.expand-button:hover {
    background: rgba(0, 0, 0, 0.1);
    color: #000;
}

.loading-state,
.error-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 300px;
    gap: 16px;
    background: rgba(255, 255, 255, 0.8);
    backdrop-filter: blur(10px);
    -webkit-backdrop-filter: blur(10px);
    border: 1px solid rgba(0, 0, 0, 0.1);
    border-radius: 8px;
    margin: 20px;
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

.main-announcement {
    border-left: 4px solid #4caf50;
}

.main-announcement .announcement-card-header {
    background: rgba(76, 175, 80, 0.1);
}

.release-announcement {
  border-left: 4px solid #ff9800;
}

.release-announcement .announcement-card-header {
  background: rgba(255, 152, 0, 0.1);
}

.other-announcements {
    border-left: 4px solid #2196f3;
}

.other-announcements .announcement-card-header {
    background: rgba(33, 150, 243, 0.1);
}

@media (prefers-color-scheme: dark) {
    .announcement-window {
        background: #2f2f2f;
    }

    .announcement-container {
        color: #fff;
    }

    .announcement-header {
        background: rgba(0, 0, 0, 0.8);
        border: 1px solid rgba(255, 255, 255, 0.2);
        color: #fff;
    }

    .announcement-title {
        color: #fff;
    }

    .announcement-card {
        background: rgba(0, 0, 0, 0.8);
        border: 1px solid rgba(255, 255, 255, 0.2);
    }

    .announcement-card-header {
        background: rgba(255, 255, 255, 0.1);
        border-bottom: 1px solid rgba(255, 255, 255, 0.2);
        color: #fff;
    }

    .announcement-card-header:hover {
        background: rgba(255, 255, 255, 0.2);
    }

    .loading-state,
    .error-state {
        background: rgba(0, 0, 0, 0.8);
        border: 1px solid rgba(255, 255, 255, 0.2);
    }

    .main-announcement .announcement-card-header {
        background: rgba(76, 175, 80, 0.2);
    }

    .release-announcement .announcement-card-header {
        background: rgba(255, 152, 0, 0.2);
    }

    .other-announcements .announcement-card-header {
        background: rgba(33, 150, 243, 0.2);
    }

    .announcement-name {
        color: #fff;
    }

    .announcement-content {
        color: #fff;
    }

    .announcement-content :deep(h1),
    .announcement-content :deep(h2),
    .announcement-content :deep(h3),
    .announcement-content :deep(h4),
    .announcement-content :deep(h5),
    .announcement-content :deep(h6) {
        color: #fff;
    }

    .announcement-content :deep(code) {
        background: rgba(255, 255, 255, 0.1);
    }

    .announcement-content :deep(pre) {
        background: rgba(255, 255, 255, 0.05);
    }

    .divider {
        background: rgba(255, 255, 255, 0.2);
    }

    .loading-text,
    .error-text {
        color: #ccc;
    }

    .refresh-button {
        background: rgba(255, 255, 255, 0.1);
        color: #fff;
    }

    .refresh-button:hover:not(:disabled) {
        background: rgba(255, 255, 255, 0.2);
    }

    .expand-button {
        color: #ccc;
    }

    .expand-button:hover {
        background: rgba(255, 255, 255, 0.1);
        color: #fff;
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
