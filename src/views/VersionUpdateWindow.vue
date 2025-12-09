<template>
    <div class="version-update-window" ref="windowRef">
        <div class="version-update-container">
            <!-- 标题 -->
            <div class="update-header">
                <h2 class="update-title">{{ t('versionUpdate.title') }}</h2>
                <div class="region-selector">
                    <label class="region-label">{{ t('versionUpdate.region') }}:</label>
                    <CustomDropdown
                        v-model="downloadRegion"
                        :options="regionOptions"
                    />
                </div>
            </div>

            <!-- 加载状 -->
            <div class="loading-state" v-if="loading">
                <RefreshCw class="spinning loading-icon" :size="32" />
                <span class="loading-text">{{ t('versionUpdate.loading') }}</span>
            </div>

            <!-- 错误状 -->
            <div class="error-state" v-else-if="error">
                <AlertCircle :size="32" class="error-icon" />
                <span class="error-text">{{ t('versionUpdate.error') }}</span>
                <button class="retry-button" @click="loadVersionInfo">
                    {{ t('versionUpdate.retry') }}
                </button>
            </div>

            <!-- 版本卡片 -->
            <div class="versions-content" v-else>
                <!-- 当前版本区域 -->
                <div class="version-section">
                    <h3 class="section-title">{{ t('versionUpdate.currentVersion') }}</h3>
                    <div class="version-cards">
                        <!-- 当前前端版本 -->
                        <div class="version-card">
                            <img :src="communityIcon" alt="Community" class="version-icon" />
                            <div class="card-content">
                                <div class="version-info">
                                    <div class="version-type-label">{{ t('versionUpdate.frontend') }}</div>
                                    <div class="version-details">
                                        <span class="version-number">{{ currentVersions.frontend }}</span>
                                        <span class="status-badge latest">{{ t('versionUpdate.status.latest') }}</span>
                                        <span class="divider">|</span>
                                        <span class="channel-badge" :class="getFrontendChannelClass()">
                                            {{ getFrontendChannelText() }}
                                        </span>
                                        <span class="divider">|</span>
                                        <span class="edition-badge">{{ t('versionUpdate.status.community') }}</span>
                                    </div>
                                </div>
                            </div>
                        </div>

                        <!-- 当前后端版本 -->
                        <div class="version-card">
                            <img :src="communityIcon" alt="Community" class="version-icon" />
                            <div class="card-content">
                                <div class="version-info">
                                    <div class="version-type-label">{{ t('versionUpdate.backend') }}</div>
                                    <div class="version-details">
                                        <span class="version-number">{{ currentVersions.backend }}</span>
                                        <span :class="['status-badge', getBackendStatus()]">
                                            {{ getBackendStatusText() }}
                                        </span>
                                        <span class="divider">|</span>
                                        <span class="channel-badge" :class="getBackendChannelClass()">
                                            {{ getBackendChannelText() }}
                                        </span>
                                        <span class="divider">|</span>
                                        <span class="edition-badge">{{ t('versionUpdate.status.community') }}</span>
                                    </div>
                                </div>

                            </div>
                        </div>
                    </div>
                </div>

                <!-- 可用版本区域 -->
                <div class="version-section">
                    <h3 class="section-title">{{ t('versionUpdate.availableVersions') }}</h3>
                    <div class="version-cards">
                        <!-- Stable Beta 渠道 -->
                        <template v-for="channel in (['stable', 'beta'] as const)" :key="channel">
                            <!-- 前端版本 -->
                            <div class="version-card">
                                <img :src="communityIcon" alt="Community" class="version-icon" />
                                <div class="card-content">
                                    <div class="version-info">
                                        <div class="version-type-label">{{ t('versionUpdate.frontend') }} - {{ t(`versionUpdate.channels.${channel}`) }}</div>
                                        <div class="version-details">
                                            <span class="version-number">{{ remoteVersions[channel]?.frontend || 'N/A' }}</span>
                                            <template v-if="shouldShowStatus(channel)">
                                                <span :class="['status-badge', getRemoteStatus(channel, 'frontend')]">
                                                    {{ getRemoteStatusText(channel, 'frontend') }}
                                                </span>
                                            </template>
                                            <span class="divider">|</span>
                                            <span class="edition-badge">{{ t('versionUpdate.status.community') }}</span>
                                        </div>
                                    </div>
                                    <div class="card-actions">
                                        <button class="action-button primary" @click="downloadAndInstall(channel, 'frontend')">
                                            <Download :size="16" />
                                            {{ t('versionUpdate.actions.downloadAndInstall') }}
                                        </button>
                                        <button 
                                            :class="['action-button', 'secondary', { subscribed: isSubscribed(channel, 'frontend') }]" 
                                            @click="subscribe(channel, 'frontend')"
                                        >
                                            <Bookmark :size="16" :fill="isSubscribed(channel, 'frontend') ? 'currentColor' : 'none'" />
                                            {{ isSubscribed(channel, 'frontend') ? t('versionUpdate.subscribed') : t('versionUpdate.actions.subscribe') }}
                                        </button>
                                    </div>
                                </div>
                            </div>

                            <!-- 后端版本 -->
                            <div class="version-card">
                                <img :src="communityIcon" alt="Community" class="version-icon" />
                                <div class="card-content">
                                    <div class="version-info">
                                        <div class="version-type-label">{{ t('versionUpdate.backend') }} - {{ t(`versionUpdate.channels.${channel}`) }}</div>
                                        <div class="version-details">
                                            <span class="version-number">{{ remoteVersions[channel]?.backend || 'N/A' }}</span>
                                            <template v-if="shouldShowStatus(channel)">
                                                <span :class="['status-badge', getRemoteStatus(channel, 'backend')]">
                                                    {{ getRemoteStatusText(channel, 'backend') }}
                                                </span>
                                            </template>
                                            <span class="divider">|</span>
                                            <span class="edition-badge">{{ t('versionUpdate.status.community') }}</span>
                                        </div>
                                    </div>
                                    <div class="card-actions">
                                        <button class="action-button primary" @click="downloadAndInstall(channel, 'backend')">
                                            <Download :size="16" />
                                            {{ t('versionUpdate.actions.downloadAndInstall') }}
                                        </button>
                                        <button 
                                            :class="['action-button', 'secondary', { subscribed: isSubscribed(channel, 'backend') }]" 
                                            @click="subscribe(channel, 'backend')"
                                        >
                                            <Bookmark :size="16" :fill="isSubscribed(channel, 'backend') ? 'currentColor' : 'none'" />
                                            {{ isSubscribed(channel, 'backend') ? t('versionUpdate.subscribed') : t('versionUpdate.actions.subscribe') }}
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </template>
                    </div>
                </div>
            </div>
        </div>

        <!-- 下载进度对话 -->
        <div v-if="downloadState.isDownloading" class="modal-overlay">
            <div class="modal-dialog download-dialog">
                <h3 class="modal-title">{{ t('versionUpdate.download.title') }}</h3>
                <div class="download-content">
                    <div class="file-name">{{ downloadState.fileName }}</div>
                    <div class="progress-bar">
                        <div class="progress-fill" :style="{ width: downloadState.progress + '%' }"></div>
                    </div>
                    <div class="progress-text">{{ downloadState.progress.toFixed(1) }}%</div>
                    <div class="download-info">
                        <span>{{ formatBytes(downloadState.downloaded) }} / {{ formatBytes(downloadState.total) }}</span>
                        <span>{{ t('versionUpdate.download.speed') }}: {{ formatBytes(downloadState.speed) }}/s</span>
                    </div>
                    <div class="download-info" v-if="downloadState.speed > 0">
                        <span>{{ t('versionUpdate.download.remaining') }}: {{ t('versionUpdate.download.about') }} {{ formatTime(downloadState.remaining) }}</span>
                    </div>
                </div>
                <div class="modal-actions">
                    <button class="modal-button cancel-button" @click="cancelDownload" :disabled="!downloadState.canCancel">
                        {{ t('versionUpdate.actions.cancelDownload') }}
                    </button>
                </div>
            </div>
        </div>

        <!-- 安装确认对话 -->
        <div v-if="showInstallConfirm" class="modal-overlay">
            <div class="modal-dialog install-dialog">
                <h3 class="modal-title">{{ t('versionUpdate.install.title') }}</h3>
                <div class="install-content">
                    <p>{{ t('versionUpdate.install.confirmMessage') }}</p>
                    <div class="install-info">
                        <div class="info-row">
                            <span class="info-label">{{ t('versionUpdate.version') }}:</span>
                            <span class="info-value">{{ installInfo.version }}</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">{{ t('versionUpdate.install.channel') }}:</span>
                            <span class="info-value">{{ installInfo.channel }}</span>
                        </div>
                        <div class="info-row">
                            <span class="info-label">{{ t('versionUpdate.install.size') }}:</span>
                            <span class="info-value">{{ formatBytes(installInfo.size) }}</span>
                        </div>
                    </div>
                    <p class="install-note" v-if="installInfo.type === 'frontend'">
                        {{ t('versionUpdate.install.willExit') }}
                    </p>
                </div>
                <div class="modal-actions">
                    <button class="modal-button cancel-button" @click="cancelInstall" :disabled="installing">
                        {{ t('common.cancel') }}
                    </button>
                    <button class="modal-button confirm-button" @click="confirmInstall" :disabled="installing">
                        {{ installing ? t('versionUpdate.actions.installing') : t('versionUpdate.actions.install') }}
                    </button>
                </div>
            </div>
        </div>

        <!-- 后端更新进度对话 -->
        <div v-if="showBackendProgress" class="modal-overlay">
            <div class="modal-dialog backend-dialog">
                <h3 class="modal-title">{{ t('versionUpdate.install.backendUpdate') }}</h3>
                <div class="backend-content">
                    <div class="backend-steps">
                        <div class="step-item" :class="{ completed: backendStep >= 1 }">
                            <CheckCircle :size="20" v-if="backendStep >= 1" />
                            <RefreshCw :size="20" class="spinning" v-else />
                            <span>{{ t('versionUpdate.install.backendSteps.download') }}</span>
                        </div>
                        <div class="step-item" :class="{ completed: backendStep >= 2, active: backendStep === 2 }">
                            <CheckCircle :size="20" v-if="backendStep > 2" />
                            <RefreshCw :size="20" class="spinning" v-else-if="backendStep === 2" />
                            <div class="step-dot" v-else></div>
                            <span>{{ t('versionUpdate.install.backendSteps.stop') }}</span>
                        </div>
                        <div class="step-item" :class="{ completed: backendStep >= 3, active: backendStep === 3 }">
                            <CheckCircle :size="20" v-if="backendStep > 3" />
                            <RefreshCw :size="20" class="spinning" v-else-if="backendStep === 3" />
                            <div class="step-dot" v-else></div>
                            <span>{{ t('versionUpdate.install.backendSteps.extract') }}</span>
                        </div>
                        <div class="step-item" :class="{ completed: backendStep >= 4, active: backendStep === 4 }">
                            <CheckCircle :size="20" v-if="backendStep > 4" />
                            <RefreshCw :size="20" class="spinning" v-else-if="backendStep === 4" />
                            <div class="step-dot" v-else></div>
                            <span>{{ t('versionUpdate.install.backendSteps.restart') }}</span>
                        </div>
                    </div>
                    <p class="backend-note">{{ t('versionUpdate.install.pleaseWait') }}</p>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted, computed, watch } from "vue";
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { RefreshCw, AlertCircle, CheckCircle, Download, Bookmark } from 'lucide-vue-next';
import { useTranslation } from "../composables/useTranslation";
import CustomDropdown from '../components/CustomDropdown.vue';
import communityIcon from '../assets/SRAico.png';

const { t } = useTranslation();

const windowRef = ref<HTMLElement | null>(null);
const loading = ref(true);
const error = ref(false);

// 下载状态
const downloadState = ref({
    isDownloading: false,
    progress: 0,
    downloaded: 0,
    total: 0,
    speed: 0,
    remaining: 0,
    fileName: '',
    filePath: '',
    canCancel: true // 是否可以取消下载
});

// 安装状态
const showInstallConfirm = ref(false);
const installing = ref(false);
const installInfo = ref({
    version: '',
    channel: '',
    size: 0,
    type: '' as 'frontend' | 'backend',
    filePath: ''
});

// 后端更新进度
const showBackendProgress = ref(false);
const backendStep = ref(0);

let unlistenDownload: (() => void) | null = null;
let unlistenBackendProgress: (() => void) | null = null;

interface VersionInfo {
    frontend: string;
    backend: string;
}

interface RemoteVersions {
    stable: VersionInfo;
    beta: VersionInfo;
}

const currentVersions = ref<VersionInfo>({
    frontend: '0.1.0',
    backend: '0.1.0'
});

const remoteVersions = ref<RemoteVersions>({
    stable: { frontend: '0.1.0', backend: '0.1.0' },
    beta: { frontend: '0.1.1', backend: '0.1.1' }
});

const currentChannel = ref<'stable' | 'beta'>('stable');

// 下载区域
const downloadRegion = ref<'china' | 'global'>('china');

// 区域选项
const regionOptions = computed(() => [
    { label: t('versionUpdate.regions.china').value, value: 'china' },
    { label: t('versionUpdate.regions.global').value, value: 'global' }
]);

// 完整的远程版本数据（包含download-url）
const fullRemoteData = ref<any>(null);

// 订阅信息（前端和后端分别订阅）
const subscription = ref<{ 
    frontend?: { channel: string; version: string };
    backend?: { channel: string; version: string };
} | null>(null);

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

// 加载版本信息
const loadVersionInfo = async () => {
    loading.value = true;
    error.value = false;

    try {
        // 1. 获取前端版本（从 Tauri 配置）
        try {
            const version = await invoke<string>('get_frontend_version');
            currentVersions.value.frontend = version || t('versionUpdate.unknown').value;
        } catch (err) {
            console.error('Failed to load frontend version:', err);
            currentVersions.value.frontend = t('versionUpdate.unknown').value;
        }

        // 2. 获取后端版本和渠道（从 StarRailAssistant/version.json）
        try {
            const backendVersion = await invoke<{ version: string; channel: string }>('get_backend_version');
            currentVersions.value.backend = backendVersion.version;
            currentChannel.value = backendVersion.channel as 'stable' | 'beta';
        } catch (err) {
            console.error('Failed to load backend version:', err);
            currentVersions.value.backend = t('versionUpdate.unknown').value;
            currentChannel.value = 'stable';
        }

        // 3. 获取远程版本信息
        try {
            const data = await invoke<any>('get_remote_versions');
            
            console.log('Remote version data:', data);
            
            // 保存完整数据
            fullRemoteData.value = data;
            
            // 解析远程版本数据
            const latestVersions = data['latest-version'];
            
            if (!latestVersions || !Array.isArray(latestVersions)) {
                throw new Error('Invalid remote version data structure');
            }
            
            // 找到 stable 和 beta 版本
            const stableItem = latestVersions.find((item: any) => item.stable);
            const betaItem = latestVersions.find((item: any) => item.beta);
            
            console.log('Stable item:', stableItem);
            console.log('Beta item:', betaItem);
            
            if (stableItem && stableItem.stable) {
                const stableData = stableItem.stable;
                remoteVersions.value.stable = {
                    frontend: stableData.frontend?.version || '0.0.0',
                    backend: stableData.backend?.version || '0.0.0'
                };
                console.log('Parsed stable versions:', remoteVersions.value.stable);
            }
            
            if (betaItem && betaItem.beta) {
                const betaData = betaItem.beta;
                remoteVersions.value.beta = {
                    frontend: betaData.frontend?.version || '0.0.0',
                    backend: betaData.backend?.version || '0.0.0'
                };
                console.log('Parsed beta versions:', remoteVersions.value.beta);
            }
        } catch (err) {
            console.error('Failed to load remote version info:', err);
            // 使用默认值
            remoteVersions.value = {
                stable: { frontend: '0.0.0', backend: '0.0.0' },
                beta: { frontend: '0.0.0', backend: '0.0.0' }
            };
        }

        // 4. 加载订阅信息
        try {
            const sub = await invoke<any>('get_subscription');
            subscription.value = sub;
            console.log('Subscription:', sub);
        } catch (err) {
            console.error('Failed to load subscription:', err);
        }

        // 5. 加载下载区域设置
        try {
            const settings = await invoke<any>('load_app_settings');
            if (settings.download_region) {
                downloadRegion.value = settings.download_region as 'china' | 'global';
                console.log('Loaded download region:', settings.download_region);
            }
        } catch (err) {
            console.error('Failed to load download region:', err);
        }
    } catch (err) {
        console.error('Failed to load version info:', err);
        error.value = true;
    } finally {
        loading.value = false;
    }
};

// 比较版本号
// 新规则：最后一位数字不为0表示测试版（如0.1.1），最后一位为0表示正式版（如0.2.0）
const compareVersions = (v1: string, v2: string): number => {
    const parts1 = v1.split('.').map(Number);
    const parts2 = v2.split('.').map(Number);
    
    const maxLen = Math.max(parts1.length, parts2.length);
    
    for (let i = 0; i < maxLen; i++) {
        const p1 = parts1[i] || 0;
        const p2 = parts2[i] || 0;
        
        if (p1 > p2) return 1;
        if (p1 < p2) return -1;
    }
    
    return 0;
};

// 判断版本是否为测试版（最后一位不为0）
const isBetaVersion = (version: string): boolean => {
    const parts = version.split('.');
    if (parts.length === 0) return false;
    const lastPart = parseInt(parts[parts.length - 1]);
    return !isNaN(lastPart) && lastPart !== 0;
};

// 获取前端渠道文本
const getFrontendChannelText = (): string => {
    const version = currentVersions.value.frontend;
    if (version === t('versionUpdate.unknown').value) {
        return t('versionUpdate.channels.stable').value;
    }
    return isBetaVersion(version) 
        ? t('versionUpdate.channels.beta').value 
        : t('versionUpdate.channels.stable').value;
};

// 获取前端渠道样式类
const getFrontendChannelClass = (): string => {
    const version = currentVersions.value.frontend;
    if (version === t('versionUpdate.unknown').value) {
        return 'channel-stable';
    }
    return isBetaVersion(version) ? 'channel-beta' : 'channel-stable';
};

// 获取后端渠道文本
const getBackendChannelText = (): string => {
    const version = currentVersions.value.backend;
    if (version === t('versionUpdate.unknown').value) {
        return t('versionUpdate.channels.stable').value;
    }
    return isBetaVersion(version) 
        ? t('versionUpdate.channels.beta').value 
        : t('versionUpdate.channels.stable').value;
};

// 获取后端渠道样式类
const getBackendChannelClass = (): string => {
    const version = currentVersions.value.backend;
    if (version === t('versionUpdate.unknown').value) {
        return 'channel-stable';
    }
    return isBetaVersion(version) ? 'channel-beta' : 'channel-stable';
};

// 获取后端状态（与所有渠道中的最高版本比较）
const getBackendStatus = (): string => {
    const current = currentVersions.value.backend;
    
    // 如果后端版本未知，返回 missing 状态
    if (current === t('versionUpdate.unknown').value) {
        return 'missing';
    }
    
    // 始终与 stable 渠道的最新版本比较
    const latestStable = remoteVersions.value.stable.backend;
    return compareVersions(current, latestStable) < 0 ? 'outdated' : 'latest';
};

const getBackendStatusText = (): string => {
    const status = getBackendStatus();
    if (status === 'missing') {
        return t('versionUpdate.status.missing').value;
    }
    return status === 'latest' 
        ? t('versionUpdate.status.latest').value 
        : t('versionUpdate.status.outdated').value;
};

// 判断是否应该显示状态标签
const shouldShowStatus = (channel: 'stable' | 'beta'): boolean => {
    // stable 渠道不显示状态
    // beta 渠道显示状态
    return channel === 'beta';
};

// 获取远程版本状态（仅用于 beta 渠道）
const getRemoteStatus = (channel: 'stable' | 'beta', type: 'frontend' | 'backend'): string => {
    if (channel !== 'beta') {
        return 'latest'; // stable 不显示，这里返回值不重要
    }
    
    // Beta 渠道：比较 beta 版本和 stable 版本
    const betaVersion = remoteVersions.value.beta[type];
    const stableVersion = remoteVersions.value.stable[type];
    
    // 比较版本号
    const result = compareVersions(betaVersion, stableVersion);
    
    // beta 版本 >= stable 版本 → 最新
    // beta 版本 < stable 版本 → 过期
    return result >= 0 ? 'latest' : 'outdated';
};

const getRemoteStatusText = (channel: 'stable' | 'beta', type: 'frontend' | 'backend'): string => {
    return getRemoteStatus(channel, type) === 'latest'
        ? t('versionUpdate.status.latest').value
        : t('versionUpdate.status.outdated').value;
};

// 格式化字节
const formatBytes = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round(bytes / Math.pow(k, i) * 100) / 100 + ' ' + sizes[i];
};

// 格式化时间
const formatTime = (seconds: number): string => {
    if (seconds < 60) {
        return Math.round(seconds) + ' ' + t('versionUpdate.download.seconds').value;
    }
    const minutes = Math.floor(seconds / 60);
    return minutes + ' ' + t('versionUpdate.download.minutes').value;
};

// 获取下载URL
const getDownloadUrl = (channel: 'stable' | 'beta', type: string): string => {
    if (!fullRemoteData.value) return '';
    
    const latestVersions = fullRemoteData.value['latest-version'];
    if (!latestVersions || !Array.isArray(latestVersions)) return '';
    
    const channelItem = latestVersions.find((item: any) => item[channel]);
    if (!channelItem || !channelItem[channel]) return '';
    
    const channelData = channelItem[channel];
    const typeData = type === 'frontend' ? channelData.frontend : channelData.backend;
    
    if (!typeData || !typeData['download-url']) return '';
    
    const downloadUrls = typeData['download-url'];
    let url = downloadUrls[downloadRegion.value] || downloadUrls.china || '';
    
    // 如果URL中包含{}占位符，替换为版本号
    if (url.includes('{}')) {
        const version = typeData.version || '';
        url = url.replace('{}', version);
    }
    
    return url;
};

// 获取文件名
const getFileName = (channel: 'stable' | 'beta', type: string): string => {
    const version = type === 'frontend' 
        ? remoteVersions.value[channel].frontend 
        : remoteVersions.value[channel].backend;
    
    if (type === 'frontend') {
        return `StarRailAssistant_${version}_Setup.exe`;
    } else {
        return `StarRailAssistant-Core_v${version}.zip`;
    }
};

// 下载和安装操作
const downloadAndInstall = async (channel: 'stable' | 'beta', type: string) => {
    // 防止并发下载
    if (downloadState.value.isDownloading) {
        window.showNotification?.(t('versionUpdate.download.alreadyDownloading').value, 2000);
        return;
    }
    
    try {
        // 1. 验证下载 URL
        const downloadUrl = getDownloadUrl(channel, type);
        if (!downloadUrl) {
            window.showNotification?.(t('versionUpdate.download.urlNotFound').value, 3000);
            await window.logToConsole?.('前端', 'ERR', `${t('versionUpdate.download.urlNotFound').value}: ${channel} ${type}`);
            return;
        }
        
        const fileName = getFileName(channel, type);
        const version = type === 'frontend' 
            ? remoteVersions.value[channel].frontend 
            : remoteVersions.value[channel].backend;
        
        // 2. 获取文件大小并检查磁盘空间
        if (!fullRemoteData.value) {
            window.showNotification?.(t('versionUpdate.download.versionInfoNotLoaded').value, 3000);
            return;
        }
        
        const latestVersions = fullRemoteData.value['latest-version'];
        const channelItem = latestVersions?.find((item: any) => item[channel]);
        const channelData = channelItem?.[channel];
        const typeData = type === 'frontend' ? channelData?.frontend : channelData?.backend;
        const fileSize = typeData?.size || 0;
        
        if (fileSize > 0) {
            try {
                const hasSpace = await invoke<boolean>('check_disk_space', {
                    requiredBytes: fileSize * 1.2 // 预留 20% 额外空间
                });
                
                if (!hasSpace) {
                    window.showNotification?.(t('versionUpdate.download.insufficientSpace').value, 3000);
                    await window.logToConsole?.('前端', 'ERR', t('versionUpdate.download.insufficientSpaceBytes').value.replace('{size}', String(fileSize)));
                    return;
                }
            } catch (error) {
                console.warn('Failed to check disk space:', error);
                // 继续下载，不阻止
            }
        }
        
        // 3. 显示下载对话框
        downloadState.value = {
            isDownloading: true,
            progress: 0,
            downloaded: 0,
            total: fileSize,
            speed: 0,
            remaining: 0,
            fileName,
            filePath: '',
            canCancel: true
        };
        
        // 4. 监听下载进度
        if (!unlistenDownload) {
            unlistenDownload = await listen<any>('download-progress', (event) => {
                const data = event.payload;
                downloadState.value.progress = data.percentage;
                downloadState.value.downloaded = data.downloaded;
                downloadState.value.total = data.total;
                downloadState.value.speed = data.speed;
                
                // 计算剩余时间
                if (data.speed > 0) {
                    const remaining = (data.total - data.downloaded) / data.speed;
                    downloadState.value.remaining = remaining;
                }
            });
        }
        
        // 5. 开始下载
        await window.logToConsole?.('前端', 'DEBUG', `下载链接: ${downloadUrl}`);
        await window.logToConsole?.('前端', 'INFO', t('versionUpdate.download.starting').value.replace('{fileName}', fileName).replace('{channel}', channel));
        
        const filePath = await invoke<string>('download_update', {
            downloadUrl,
            fileName,
            updateType: type
        });
        
        downloadState.value.filePath = filePath;
        downloadState.value.isDownloading = false;
        downloadState.value.canCancel = false;
        
        await window.logToConsole?.('前端', 'SUCCESS', t('versionUpdate.download.downloadComplete').value.replace('{fileName}', fileName));
        
        // 6. 准备安装信息
        installInfo.value = {
            version,
            channel: t(`versionUpdate.channels.${channel}`).value,
            size: downloadState.value.total,
            type: type as 'frontend' | 'backend',
            filePath
        };
        
        // 7. 显示安装确认对话框
        showInstallConfirm.value = true;
        
    } catch (error) {
        console.error('Download failed:', error);
        downloadState.value.isDownloading = false;
        downloadState.value.canCancel = false;
        
        const errorMsg = error instanceof Error ? error.message : String(error);
        window.showNotification?.(t('versionUpdate.download.failed').value, 3000);
        await window.logToConsole?.('前端', 'ERR', t('versionUpdate.download.downloadFailed').value.replace('{error}', errorMsg));
        
        // 清理可能存在的临时文件
        if (downloadState.value.filePath) {
            try {
                await invoke('delete_temp_file', { filePath: downloadState.value.filePath });
            } catch (e) {
                console.error('Failed to cleanup temp file:', e);
            }
        }
    }
};

const cancelDownload = async () => {
    if (!downloadState.value.canCancel) {
        return;
    }
    
    downloadState.value.isDownloading = false;
    downloadState.value.canCancel = false;
    
    await window.logToConsole?.('前端', 'WARN', t('versionUpdate.download.userCancelled').value);
    
    // 清理下载的临时文件
    if (downloadState.value.filePath) {
        try {
            await invoke('delete_temp_file', { filePath: downloadState.value.filePath });
        } catch (error) {
            console.error('Failed to delete temp file:', error);
        }
    }
    
    // 清理事件监听器
    if (unlistenDownload) {
        unlistenDownload();
        unlistenDownload = null;
    }
    
    window.showNotification?.(t('versionUpdate.download.cancelled').value, 2000);
};

const cancelInstall = async () => {
    showInstallConfirm.value = false;
    
    await window.logToConsole?.('前端', 'WARN', t('versionUpdate.install.userCancelled').value);
    
    // 清理下载的文 
    if (installInfo.value.filePath) {
        try {
            await invoke('delete_temp_file', { filePath: installInfo.value.filePath });
            await window.logToConsole?.('前端', 'INFO', t('versionUpdate.install.tempFileCleaned').value);
        } catch (error) {
            console.error('Failed to delete temp file:', error);
        }
    }
};

const confirmInstall = async () => {
    installing.value = true;
    
    try {
        if (installInfo.value.type === 'frontend') {
            // 前端更新：退出应用并安装
            await window.logToConsole?.('前端', 'INFO', t('versionUpdate.install.startingFrontend').value.replace('{version}', installInfo.value.version));
            
            await invoke('install_frontend_update', {
                installerPath: installInfo.value.filePath
            });
            // 应用会自动退 
        } else {
            // 后端更新：显示进度对话框
            showInstallConfirm.value = false;
            showBackendProgress.value = true;
            backendStep.value = 1;
            
            await window.logToConsole?.('前端', 'INFO', t('versionUpdate.install.startingBackend').value.replace('{version}', installInfo.value.version));
            
            // 监听后端更新进度
            if (!unlistenBackendProgress) {
                unlistenBackendProgress = await listen<string>('backend-update-progress', (event) => {
                    const step = event.payload;
                    if (step === 'stopping') {
                        backendStep.value = 2;
                        window.logToConsole?.('前端', 'INFO', t('versionUpdate.install.stoppingBackend').value);
                    }
                    else if (step === 'extracting') {
                        backendStep.value = 3;
                        window.logToConsole?.('前端', 'INFO', t('versionUpdate.install.extractingFiles').value);
                    }
                    else if (step === 'restarting') {
                        backendStep.value = 4;
                        window.logToConsole?.('前端', 'INFO', t('versionUpdate.install.restartingBackend').value);
                    }
                    else if (step === 'completed') {
                        backendStep.value = 5;
                        window.logToConsole?.('前端', 'SUCCESS', t('versionUpdate.install.backendComplete').value);
                        setTimeout(() => {
                            showBackendProgress.value = false;
                            window.showNotification?.(t('versionUpdate.install.success').value, 3000);
                            // 刷新版本信息
                            loadVersionInfo();
                        }, 1000);
                    }
                });
            }
            
            // 开始后端更 
            await invoke('install_backend_update', {
                zipPath: installInfo.value.filePath
            });
        }
    } catch (error) {
        console.error('Install failed:', error);
        installing.value = false;
        showBackendProgress.value = false;
        
        const errorMsg = error instanceof Error ? error.message : String(error);
        window.showNotification?.(t('versionUpdate.install.failed').value, 3000);
        await window.logToConsole?.('前端', 'ERR', t('versionUpdate.install.installFailed').value.replace('{error}', errorMsg));
    }
};

const subscribe = async (channel: 'stable' | 'beta', type: 'frontend' | 'backend') => {
    try {
        const version = type === 'frontend' 
            ? remoteVersions.value[channel].frontend 
            : remoteVersions.value[channel].backend;
        
        await invoke('save_subscription', {
            type,
            channel,
            version
        });
        
        // 更新订阅状 
        if (!subscription.value) {
            subscription.value = {};
        }
        subscription.value[type] = { channel, version };
        
        const typeName = type === 'frontend' ? t('versionUpdate.frontend').value : t('versionUpdate.backend').value;
        const channelName = t(`versionUpdate.channels.${channel}`).value;
        window.showNotification?.(
            t('versionUpdate.subscribe.success').value.replace('{type}', typeName).replace('{channel}', channelName).replace('{version}', version),
            3000
        );
        
        await window.logToConsole?.('前端', 'INFO', t('versionUpdate.subscribe.logMessage').value.replace('{type}', typeName).replace('{channel}', channel).replace('{version}', version));
    } catch (error) {
        console.error('Subscribe failed:', error);
        window.showNotification?.(t('versionUpdate.subscribe.failed').value, 3000);
    }
};

// 检查某个类型是否已订阅某个渠道
const isSubscribed = (channel: 'stable' | 'beta', type: 'frontend' | 'backend'): boolean => {
    return subscription.value?.[type]?.channel === channel;
};

// 监听下载区域变化并保 
watch(downloadRegion, async (newRegion) => {
    try {
        await invoke('save_download_region', { region: newRegion });
        console.log('Download region saved:', newRegion);
    } catch (error) {
        console.error('Failed to save download region:', error);
    }
});

onMounted(() => {
    loadWallpaper();
    loadVersionInfo();
});

onUnmounted(() => {
    // 清理事件监听
    if (unlistenDownload) {
        unlistenDownload();
        unlistenDownload = null;
    }
    if (unlistenBackendProgress) {
        unlistenBackendProgress();
        unlistenBackendProgress = null;
    }
    
    // 如果正在下载，记录警 
    if (downloadState.value.isDownloading) {
        console.warn('Component unmounted while download in progress');
    }
});
</script>

<style scoped>
.version-update-window {
  width: 100vw;
  height: 100vh;
  background: url('../assets/background-lt.jpg') no-repeat center center;
  background-size: cover;
  overflow: hidden;
}

.version-update-container {
  height: 100%;
  padding: 16px;
  display: flex;
  flex-direction: column;
  box-sizing: border-box;
}

.update-header {
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

.update-title {
  margin: 0;
  font-size: 24px;
  font-weight: 600;
  color: #000;
}

.region-selector {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 12px;
}

.region-label {
  font-size: 14px;
  font-weight: 500;
  color: rgba(0, 0, 0, 0.8);
  white-space: nowrap;
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

.retry-button {
  padding: 8px 16px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  background: rgba(0, 0, 0, 0.1);
  color: #000;
}

.retry-button:hover {
  background: rgba(0, 0, 0, 0.2);
  transform: translateY(-1px);
}

.versions-content {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 20px;
}

.versions-content::-webkit-scrollbar {
  width: 8px;
}

.versions-content::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.05);
  border-radius: 4px;
}

.versions-content::-webkit-scrollbar-thumb {
  background: rgba(0, 0, 0, 0.2);
  border-radius: 4px;
}

.versions-content::-webkit-scrollbar-thumb:hover {
  background: rgba(0, 0, 0, 0.3);
}

.version-section {
  background: rgba(255, 255, 255, 0.8);
  backdrop-filter: blur(10px);
  -webkit-backdrop-filter: blur(10px);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  padding: 20px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
}

.section-title {
  margin: 0 0 16px 0;
  font-size: 18px;
  font-weight: 600;
  color: #000;
  padding-bottom: 12px;
  border-bottom: 2px solid rgba(0, 0, 0, 0.1);
}

.version-cards {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

.version-card {
  display: flex;
  align-items: center;
  gap: 16px;
  background: rgba(255, 255, 255, 0.9);
  border: 1px solid rgba(0, 0, 0, 0.1);
  border-radius: 8px;
  padding: 12px 16px;
  transition: all 0.3s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.version-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.version-icon {
  width: 48px;
  height: 48px;
  border-radius: 8px;
  object-fit: cover;
  flex-shrink: 0;
}

.card-content {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  min-width: 0;
}

.version-info {
  display: flex;
  flex-direction: column;
  gap: 6px;
  flex: 1;
  min-width: 0;
}

.version-type-label {
  font-size: 16px;
  font-weight: 700;
  color: #000;
}

.version-details {
  display: flex;
  align-items: center;
  gap: 10px;
  flex-wrap: wrap;
}

.version-number {
  font-size: 14px;
  font-weight: 500;
  color: rgba(0, 0, 0, 0.7);
  white-space: nowrap;
}

.status-badge {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
  white-space: nowrap;
}

.status-badge.latest {
  background: rgba(76, 175, 80, 0.2);
  color: #2e7d32;
}

.status-badge.outdated {
  background: rgba(255, 152, 0, 0.2);
  color: #e65100;
}

.status-badge.missing {
  background: rgba(244, 67, 54, 0.2);
  color: #c62828;
}

.divider {
  color: rgba(0, 0, 0, 0.3);
  font-weight: 300;
}

.channel-badge {
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 12px;
  font-weight: 600;
  text-transform: uppercase;
  white-space: nowrap;
}

.channel-badge.channel-stable {
  background: rgba(76, 175, 80, 0.2);
  color: #2e7d32;
}

.channel-badge.channel-beta {
  background: rgba(156, 39, 176, 0.2);
  color: #6a1b9a;
}

.edition-badge {
  font-size: 12px;
  color: rgba(0, 0, 0, 0.6);
  font-weight: 500;
  white-space: nowrap;
}

.card-actions {
  display: flex;
  gap: 8px;
  flex-shrink: 0;
}

.action-button {
  display: flex;
  align-items: center;
  gap: 6px;
  padding: 6px 12px;
  border: none;
  border-radius: 6px;
  font-size: 13px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  white-space: nowrap;
}

.action-button.primary {
  background: rgba(33, 150, 243, 0.8);
  color: white;
}

.action-button.primary:hover {
  background: rgba(33, 150, 243, 0.9);
  transform: translateY(-1px);
}

.action-button.secondary {
  background: rgba(156, 39, 176, 0.8);
  color: white;
}

.action-button.secondary:hover {
  background: rgba(156, 39, 176, 0.9);
  transform: translateY(-1px);
}

.action-button:not(.primary):not(.secondary) {
  background: rgba(76, 175, 80, 0.8);
  color: white;
}

.action-button:not(.primary):not(.secondary):hover {
  background: rgba(76, 175, 80, 0.9);
  transform: translateY(-1px);
}

@media (prefers-color-scheme: dark) {
  .update-header {
    background: rgba(0, 0, 0, 0.8);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .update-title {
    color: #f1f5f9;
  }

  .region-label {
    color: rgba(255, 255, 255, 0.8);
  }

  .loading-state,
  .error-state {
    background: rgba(0, 0, 0, 0.8);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .loading-text,
  .error-text {
    color: #cbd5e1;
  }

  .loading-icon,
  .error-icon {
    color: #94a3b8;
  }

  .retry-button {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
  }

  .retry-button:hover {
    background: rgba(255, 255, 255, 0.2);
  }

  .version-section {
    background: rgba(0, 0, 0, 0.8);
    border: 1px solid rgba(255, 255, 255, 0.2);
  }

  .section-title {
    color: #f1f5f9;
    border-bottom-color: rgba(255, 255, 255, 0.2);
  }

  .version-card {
    background: rgba(255, 255, 255, 0.05);
    border: 1px solid rgba(255, 255, 255, 0.1);
  }

  .version-card:hover {
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.3);
  }

  .version-type-label {
    color: #f1f5f9;
  }

  .version-number {
    color: rgba(255, 255, 255, 0.7);
  }

  .status-badge.latest {
    background: rgba(76, 175, 80, 0.3);
    color: #81c784;
  }

  .status-badge.outdated {
    background: rgba(255, 152, 0, 0.3);
    color: #ffb74d;
  }

  .status-badge.missing {
    background: rgba(244, 67, 54, 0.3);
    color: #ef5350;
  }

  .divider {
    color: rgba(255, 255, 255, 0.3);
  }

  .channel-badge.channel-stable {
    background: rgba(76, 175, 80, 0.3);
    color: #81c784;
  }

  .channel-badge.channel-beta {
    background: rgba(156, 39, 176, 0.3);
    color: #ba68c8;
  }

  .edition-badge {
    color: rgba(255, 255, 255, 0.6);
  }

  .versions-content::-webkit-scrollbar-track {
    background: rgba(255, 255, 255, 0.05);
  }

  .versions-content::-webkit-scrollbar-thumb {
    background: rgba(255, 255, 255, 0.2);
  }

  .versions-content::-webkit-scrollbar-thumb:hover {
    background: rgba(255, 255, 255, 0.3);
  }
}

/* 模态对话框样式 */
.modal-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(4px);
}

.modal-dialog {
  background: rgba(255, 255, 255, 0.95);
  border-radius: 12px;
  padding: 24px;
  min-width: 400px;
  max-width: 500px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
}

.modal-title {
  margin: 0 0 20px 0;
  font-size: 20px;
  font-weight: 600;
  color: #000;
}

.modal-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 24px;
}

.modal-button {
  padding: 8px 20px;
  border: none;
  border-radius: 6px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
}

.modal-button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.modal-button.cancel-button {
  background: rgba(0, 0, 0, 0.1);
  color: #000;
}

.modal-button.cancel-button:hover:not(:disabled) {
  background: rgba(0, 0, 0, 0.2);
}

.modal-button.confirm-button {
  background: rgba(33, 150, 243, 0.9);
  color: white;
}

.modal-button.confirm-button:hover:not(:disabled) {
  background: rgba(33, 150, 243, 1);
}

/* 下载对话 */
.download-content {
  text-align: center;
}

.file-name {
  font-size: 14px;
  color: rgba(0, 0, 0, 0.7);
  margin-bottom: 16px;
  word-break: break-all;
}

.progress-bar {
  width: 100%;
  height: 8px;
  background: rgba(0, 0, 0, 0.1);
  border-radius: 4px;
  overflow: hidden;
  margin-bottom: 8px;
}

.progress-fill {
  height: 100%;
  background: linear-gradient(90deg, #2196f3, #21cbf3);
  transition: width 0.3s ease;
}

.progress-text {
  font-size: 24px;
  font-weight: 700;
  color: #000;
  margin-bottom: 12px;
}

.download-info {
  display: flex;
  justify-content: space-between;
  font-size: 13px;
  color: rgba(0, 0, 0, 0.6);
  margin-bottom: 8px;
}

/* 安装对话 */
.install-content p {
  margin: 0 0 16px 0;
  color: rgba(0, 0, 0, 0.8);
}

.install-info {
  background: rgba(0, 0, 0, 0.05);
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 16px;
}

.info-row {
  display: flex;
  justify-content: space-between;
  margin-bottom: 8px;
}

.info-row:last-child {
  margin-bottom: 0;
}

.info-label {
  font-weight: 500;
  color: rgba(0, 0, 0, 0.7);
}

.info-value {
  font-weight: 600;
  color: #000;
}

.install-note {
  font-size: 13px;
  color: rgba(0, 0, 0, 0.6);
  line-height: 1.5;
}

/* 后端更新对话 */
.backend-content {
  padding: 8px 0;
}

.backend-steps {
  display: flex;
  flex-direction: column;
  gap: 16px;
  margin-bottom: 20px;
}

.step-item {
  display: flex;
  align-items: center;
  gap: 12px;
  color: rgba(0, 0, 0, 0.4);
  transition: all 0.3s ease;
}

.step-item.active {
  color: #2196f3;
}

.step-item.completed {
  color: #4caf50;
}

.step-dot {
  width: 20px;
  height: 20px;
  border-radius: 50%;
  background: rgba(0, 0, 0, 0.1);
}

.backend-note {
  text-align: center;
  font-size: 13px;
  color: rgba(0, 0, 0, 0.6);
  margin: 0;
}

@media (prefers-color-scheme: dark) {
  .modal-dialog {
    background: rgba(30, 30, 30, 0.95);
  }

  .modal-title {
    color: #f1f5f9;
  }

  .modal-button.cancel-button {
    background: rgba(255, 255, 255, 0.1);
    color: #fff;
  }

  .modal-button.cancel-button:hover:not(:disabled) {
    background: rgba(255, 255, 255, 0.2);
  }

  .file-name {
    color: rgba(255, 255, 255, 0.7);
  }

  .progress-bar {
    background: rgba(255, 255, 255, 0.1);
  }

  .progress-text {
    color: #f1f5f9;
  }

  .download-info {
    color: rgba(255, 255, 255, 0.6);
  }

  .install-content p {
    color: rgba(255, 255, 255, 0.8);
  }

  .install-info {
    background: rgba(255, 255, 255, 0.05);
  }

  .info-label {
    color: rgba(255, 255, 255, 0.7);
  }

  .info-value {
    color: #f1f5f9;
  }

  .install-note {
    color: rgba(255, 255, 255, 0.6);
  }

  .step-item {
    color: rgba(255, 255, 255, 0.4);
  }

  .step-item.active {
    color: #64b5f6;
  }

  .step-item.completed {
    color: #81c784;
  }

  .step-dot {
    background: rgba(255, 255, 255, 0.1);
  }

  .backend-note {
    color: rgba(255, 255, 255, 0.6);
  }
}
</style>
