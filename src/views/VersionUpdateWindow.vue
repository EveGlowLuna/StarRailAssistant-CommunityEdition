<template>
    <div class="version-update-window">
        <div class="version-update-container">
            <h2 class="window-title">{{ t('home.versionUpdate') }}</h2>
            <p class="placeholder-text">版本更新功能即将实现...</p>
        </div>
    </div>
</template>

<script setup lang="ts">
import { useTranslation } from "../composables/useTranslation";

const { t } = useTranslation();
</script>

<style scoped>
.version-update-window {
  width: 100vw;
  height: 100vh;
  background: #f6f6f6;
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
        background: #2f2f2f;
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
