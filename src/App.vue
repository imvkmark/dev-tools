<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { IconCopy } from '@arco-design/web-vue/es/icon';
import { Notification } from '@arco-design/web-vue';

// 当前活动的工具
const activeTab = ref('json');

// JSON工具
const jsonInput = ref('');
const jsonOutput = ref('');
const jsonError = ref('');

// Base64工具
const base64Input = ref('');
const base64Output = ref('');
const base64Error = ref('');
const base64Mode = ref('encode');

// URL工具
const urlInput = ref('');
const urlOutput = ref('');
const urlError = ref('');
const urlMode = ref('encode');

// 时间戳工具
const timestampInput = ref('');
const timestampOutput = ref('');
const timestampError = ref('');
const timestampMode = ref('toDate');
const timestampUnit = ref('seconds');

// 正则表达式工具
const regexPattern = ref('');
const regexText = ref('');
const regexFlags = ref('g');
const regexResult = ref<any>(null);
const regexError = ref('');

// UUID工具
const generatedUuid = ref('');

// 哈希工具
const hashInput = ref('');
const hashOutput = ref('');
const hashAlgorithm = ref('md5');
const hashError = ref('');

// 工具列表
const tools = [
  { id: 'json', name: 'JSON格式化', icon: '{}' },
  { id: 'base64', name: 'Base64编解码', icon: '🔐' },
  { id: 'url', name: 'URL编解码', icon: '🔗' },
  { id: 'timestamp', name: '时间戳转换', icon: '⏰' },
  { id: 'regex', name: '正则表达式', icon: '📝' },
  { id: 'uuid', name: 'UUID生成', icon: '🆔' },
  { id: 'hash', name: '哈希计算', icon: '#️⃣' }
];

// JSON格式化
async function formatJson() {
  try {
    jsonError.value = '';
    jsonOutput.value = await invoke('format_json', { input: jsonInput.value });
  } catch (error) {
    jsonError.value = error as string;
    jsonOutput.value = '';
  }
}

// JSON压缩
async function minifyJson() {
  try {
    jsonError.value = '';
    jsonOutput.value = await invoke('minify_json', { input: jsonInput.value });
  } catch (error) {
    jsonError.value = error as string;
    jsonOutput.value = '';
  }
}

// Base64编解码
async function processBase64() {
  try {
    base64Error.value = '';
    if (base64Mode.value === 'encode') {
      base64Output.value = await invoke('encode_base64', { input: base64Input.value });
    } else {
      base64Output.value = await invoke('decode_base64', { input: base64Input.value });
    }
  } catch (error) {
    base64Error.value = error as string;
    base64Output.value = '';
  }
}

// URL编解码
async function processUrl() {
  try {
    urlError.value = '';
    if (urlMode.value === 'encode') {
      urlOutput.value = await invoke('encode_url', { input: urlInput.value });
    } else {
      urlOutput.value = await invoke('decode_url', { input: urlInput.value });
    }
  } catch (error) {
    urlError.value = error as string;
    urlOutput.value = '';
  }
}

// 时间戳转换
async function processTimestamp() {
  try {
    timestampError.value = '';
    if (timestampMode.value === 'toDate') {
      const timestamp = parseInt(timestampInput.value);
      timestampOutput.value = await invoke('timestamp_to_date', { 
        timestamp, 
        unit: timestampUnit.value 
      });
    } else {
      timestampOutput.value = String(await invoke('date_to_timestamp', { 
        dateStr: timestampInput.value, 
        unit: timestampUnit.value 
      }));
    }
  } catch (error) {
    timestampError.value = error as string;
    timestampOutput.value = '';
  }
}

// 正则表达式测试
async function testRegex() {
  try {
    regexError.value = '';
    regexResult.value = await invoke('test_regex', { 
      pattern: regexPattern.value, 
      text: regexText.value, 
      flags: regexFlags.value 
    });
  } catch (error) {
    regexError.value = error as string;
    regexResult.value = null;
  }
}

// 生成UUID
async function generateUuid() {
  generatedUuid.value = await invoke('generate_uuid');
}

// 计算哈希
async function calculateHash() {
  try {
    hashError.value = '';
    hashOutput.value = await invoke('calculate_hash', { 
      input: hashInput.value, 
      algorithm: hashAlgorithm.value 
    });
  } catch (error) {
    hashError.value = error as string;
    hashOutput.value = '';
  }
}

// 移除旧的复制成功变量，现在使用 Arco Design 通知

// 复制到剪贴板
async function copyToClipboard(text: string) {
  try {
    await navigator.clipboard.writeText(text);
    showCopySuccess();
  } catch (error) {
    console.error('复制失败:', error);
  }
}

// 显示复制成功提示
const showCopySuccess = () => {
  // 使用 Arco Design 的通知组件
  Notification.success({
    title: '复制成功',
    content: '内容已复制到剪贴板',
    duration: 2000,
    position: 'topRight'
  });
};

// 获取当前时间戳
function getCurrentTimestamp() {
  const now = Date.now();
  if (timestampUnit.value === 'seconds') {
    timestampInput.value = String(Math.floor(now / 1000));
  } else {
    timestampInput.value = String(now);
  }
}

// 获取当前日期
function getCurrentDate() {
  const now = new Date();
  timestampInput.value = now.toISOString().slice(0, 19).replace('T', ' ');
}
</script>

<template>
  <a-layout class="app">


    <!-- 侧边栏 -->
    <a-layout-sider 
      :width="300" 
      :collapsed="false" 
      class="sidebar"
      theme="light"
    >
      <div class="logo">
        <a-typography-title :heading="2" style="margin: 0; color: #1890ff;">
          🛠️ 开发工具箱
        </a-typography-title>
      </div>
      
      <a-menu 
        :selected-keys="[activeTab]" 
        @menu-item-click="activeTab = $event"
        class="nav-menu"
      >
        <a-menu-item 
          v-for="tool in tools" 
          :key="tool.id"
          class="nav-item"
        >
          <template #icon>
            <span class="tool-icon">{{ tool.icon }}</span>
          </template>
          {{ tool.name }}
        </a-menu-item>
      </a-menu>
    </a-layout-sider>

    <!-- 主内容区 -->
    <a-layout-content class="main-content">
      <!-- JSON格式化工具 -->
      <a-card v-if="activeTab === 'json'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            JSON格式化工具
          </a-typography-title>
        </template>
        <template #extra>
          <a-space>
            <a-button type="primary" @click="formatJson">格式化</a-button>
            <a-button @click="minifyJson">压缩</a-button>
          </a-space>
        </template>
        
        <a-row :gutter="24">
          <a-col :span="12">
            <div class="input-section">
              <a-typography-text strong>输入JSON:</a-typography-text>
              <a-textarea 
                v-model="jsonInput" 
                placeholder="请输入JSON字符串..."
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px;"
              />
            </div>
          </a-col>
          <a-col :span="12">
            <div class="output-section">
              <a-space style="width: 100%; justify-content: space-between;">
                <a-typography-text strong>输出结果:</a-typography-text>
                <a-button 
                  v-if="jsonOutput" 
                  type="text" 
                  size="small"
                  @click="copyToClipboard(jsonOutput)"
                >
                  <template #icon>
                    <icon-copy />
                  </template>
                  复制
                </a-button>
              </a-space>
              <a-textarea 
                v-model="jsonOutput" 
                readonly 
                placeholder="格式化后的JSON将显示在这里..."
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px;"
              />
            </div>
          </a-col>
        </a-row>
        
        <a-alert 
          v-if="jsonError" 
          type="error" 
          :message="jsonError"
          style="margin-top: 16px;"
          show-icon
        />
      </a-card>

      <!-- Base64编解码工具 -->
      <a-card v-if="activeTab === 'base64'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            Base64编解码工具
          </a-typography-title>
        </template>
        <template #extra>
          <a-space>
            <a-select v-model="base64Mode" style="width: 120px;">
              <a-option value="encode">编码</a-option>
              <a-option value="decode">解码</a-option>
            </a-select>
            <a-button type="primary" @click="processBase64">转换</a-button>
          </a-space>
        </template>
        
        <a-row :gutter="24">
          <a-col :span="12">
            <div class="input-section">
              <a-typography-text strong>输入文本:</a-typography-text>
              <a-textarea 
                v-model="base64Input" 
                :placeholder="base64Mode === 'encode' ? '请输入要编码的文本...' : '请输入要解码的Base64字符串...'"
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px;"
              />
            </div>
          </a-col>
          <a-col :span="12">
            <div class="output-section">
              <a-space style="width: 100%; justify-content: space-between;">
                <a-typography-text strong>输出结果:</a-typography-text>
                <a-button 
                  v-if="base64Output" 
                  type="text" 
                  size="small"
                  @click="copyToClipboard(base64Output)"
                >
                  <template #icon>
                    <icon-copy />
                  </template>
                  复制
                </a-button>
              </a-space>
              <a-textarea 
                v-model="base64Output" 
                readonly 
                placeholder="转换结果将显示在这里..."
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px;"
              />
            </div>
          </a-col>
        </a-row>
        
        <a-alert 
          v-if="base64Error" 
          type="error" 
          :message="base64Error"
          style="margin-top: 16px;"
          show-icon
        />
      </a-card>

      <!-- URL编解码工具 -->
      <a-card v-if="activeTab === 'url'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            URL编解码工具
          </a-typography-title>
        </template>
        <template #extra>
          <a-space>
            <a-select v-model="urlMode" style="width: 120px;">
              <a-option value="encode">编码</a-option>
              <a-option value="decode">解码</a-option>
            </a-select>
            <a-button type="primary" @click="processUrl">转换</a-button>
          </a-space>
        </template>
        
        <a-row :gutter="24">
          <a-col :span="12">
            <div class="input-section">
              <a-typography-text strong>输入URL:</a-typography-text>
              <a-textarea 
                v-model="urlInput" 
                :placeholder="urlMode === 'encode' ? '请输入要编码的URL...' : '请输入要解码的URL...'"
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px;"
              />
            </div>
          </a-col>
          <a-col :span="12">
            <div class="output-section">
              <a-space style="width: 100%; justify-content: space-between;">
                <a-typography-text strong>输出结果:</a-typography-text>
                <a-button 
                  v-if="urlOutput" 
                  type="text" 
                  size="small"
                  @click="copyToClipboard(urlOutput)"
                >
                  <template #icon>
                    <icon-copy />
                  </template>
                  复制
                </a-button>
              </a-space>
              <a-textarea 
                v-model="urlOutput" 
                readonly 
                placeholder="转换结果将显示在这里..."
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px;"
              />
            </div>
          </a-col>
        </a-row>
        
        <a-alert 
          v-if="urlError" 
          type="error" 
          :message="urlError"
          style="margin-top: 16px;"
          show-icon
        />
      </a-card>

      <!-- 时间戳转换工具 -->
      <a-card v-if="activeTab === 'timestamp'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            时间戳转换工具
          </a-typography-title>
        </template>
        <template #extra>
          <a-space>
            <a-select v-model="timestampMode" style="width: 150px;">
              <a-option value="toDate">时间戳转日期</a-option>
              <a-option value="toTimestamp">日期转时间戳</a-option>
            </a-select>
            <a-select v-model="timestampUnit" style="width: 100px;">
              <a-option value="seconds">秒</a-option>
              <a-option value="milliseconds">毫秒</a-option>
            </a-select>
            <a-button type="primary" @click="processTimestamp">转换</a-button>
          </a-space>
        </template>
        
        <a-row :gutter="24">
          <a-col :span="12">
            <div class="input-section">
              <a-space style="width: 100%; justify-content: space-between;">
                <a-typography-text strong>输入:</a-typography-text>
                <a-button 
                  v-if="timestampMode === 'toDate'" 
                  type="text" 
                  size="small"
                  @click="getCurrentTimestamp"
                >
                  当前时间戳
                </a-button>
                <a-button 
                  v-if="timestampMode === 'toTimestamp'" 
                  type="text" 
                  size="small"
                  @click="getCurrentDate"
                >
                  当前日期
                </a-button>
              </a-space>
              <a-textarea 
                v-model="timestampInput" 
                :placeholder="timestampMode === 'toDate' ? '请输入时间戳...' : '请输入日期 (YYYY-MM-DD HH:MM:SS)...'"
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px;"
              />
            </div>
          </a-col>
          <a-col :span="12">
            <div class="output-section">
              <a-space style="width: 100%; justify-content: space-between;">
                <a-typography-text strong>输出结果:</a-typography-text>
                <a-button 
                  v-if="timestampOutput" 
                  type="text" 
                  size="small"
                  @click="copyToClipboard(timestampOutput)"
                >
                  <template #icon>
                    <icon-copy />
                  </template>
                  复制
                </a-button>
              </a-space>
              <a-textarea 
                v-model="timestampOutput" 
                readonly 
                placeholder="转换结果将显示在这里..."
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px;"
              />
            </div>
          </a-col>
        </a-row>
        
        <a-alert 
          v-if="timestampError" 
          type="error" 
          :message="timestampError"
          style="margin-top: 16px;"
          show-icon
        />
      </a-card>

      <!-- 正则表达式工具 -->
      <a-card v-if="activeTab === 'regex'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            正则表达式测试工具
          </a-typography-title>
        </template>
        <template #extra>
          <a-button type="primary" @click="testRegex">测试</a-button>
        </template>
        
        <a-space direction="vertical" style="width: 100%;" :size="16">
          <a-row :gutter="16">
            <a-col :span="18">
              <a-typography-text strong>正则表达式:</a-typography-text>
              <a-input 
                v-model="regexPattern" 
                placeholder="请输入正则表达式..."
                style="margin-top: 8px;"
              />
            </a-col>
            <a-col :span="6">
              <a-typography-text strong>标志:</a-typography-text>
              <a-input 
                v-model="regexFlags" 
                placeholder="i, m, s"
                style="margin-top: 8px;"
              />
            </a-col>
          </a-row>
          
          <div>
            <a-typography-text strong>测试文本:</a-typography-text>
            <a-textarea 
              v-model="regexText" 
              placeholder="请输入要测试的文本..."
              :rows="8"
              :auto-size="{ minRows: 8, maxRows: 12 }"
              style="margin-top: 8px;"
            />
          </div>
          
          <div v-if="regexResult">
            <a-typography-text strong>匹配结果:</a-typography-text>
            <a-card style="margin-top: 8px;" size="small">
              <a-space style="margin-bottom: 12px;">
                <a-tag color="blue">匹配数量: {{ regexResult.count }}</a-tag>
                <a-tag :color="regexResult.is_match ? 'green' : 'red'">
                  {{ regexResult.is_match ? '匹配成功' : '无匹配' }}
                </a-tag>
              </a-space>
              
              <div v-if="regexResult.matches.length > 0">
                <a-space direction="vertical" style="width: 100%;" :size="8">
                  <a-card 
                    v-for="(match, index) in regexResult.matches" 
                    :key="index" 
                    size="small"
                    style="background: #f6f8fa;"
                  >
                    <a-space style="width: 100%; justify-content: space-between;">
                      <a-typography-text code>{{ match.match }}</a-typography-text>
                      <a-typography-text type="secondary">
                        位置: {{ match.start }}-{{ match.end }}
                      </a-typography-text>
                    </a-space>
                  </a-card>
                </a-space>
              </div>
            </a-card>
          </div>
        </a-space>
        
        <a-alert 
          v-if="regexError" 
          type="error" 
          :message="regexError"
          style="margin-top: 16px;"
          show-icon
        />
      </a-card>

      <!-- UUID生成工具 -->
      <a-card v-if="activeTab === 'uuid'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            UUID生成工具
          </a-typography-title>
        </template>
        <template #extra>
          <a-button type="primary" @click="generateUuid">生成UUID</a-button>
        </template>
        
        <div style="text-align: center; padding: 40px 0;">
          <a-typography-text strong style="display: block; margin-bottom: 16px;">
            生成的UUID:
          </a-typography-text>
          
          <a-input-group style="max-width: 500px; margin: 0 auto;">
            <a-input 
              v-model="generatedUuid" 
              readonly 
              placeholder="点击生成UUID..."
              style="text-align: center; font-family: monospace; font-size: 16px;"
            />
            <a-button 
              v-if="generatedUuid" 
              type="primary"
              @click="copyToClipboard(generatedUuid)"
            >
              <template #icon>
                <icon-copy />
              </template>
              复制
            </a-button>
          </a-input-group>
        </div>
      </a-card>

      <!-- 哈希计算工具 -->
      <a-card v-if="activeTab === 'hash'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            哈希计算工具
          </a-typography-title>
        </template>
        <template #extra>
          <a-space>
            <a-select v-model="hashAlgorithm" style="width: 120px;">
              <a-option value="md5">MD5</a-option>
              <a-option value="sha256">SHA256</a-option>
            </a-select>
            <a-button type="primary" @click="calculateHash">计算</a-button>
          </a-space>
        </template>
        
        <a-row :gutter="24">
          <a-col :span="12">
            <div class="input-section">
              <a-typography-text strong>输入文本:</a-typography-text>
              <a-textarea 
                v-model="hashInput" 
                placeholder="请输入要计算哈希的文本..."
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px;"
              />
            </div>
          </a-col>
          <a-col :span="12">
            <div class="output-section">
              <a-space style="width: 100%; justify-content: space-between;">
                <a-typography-text strong>哈希值:</a-typography-text>
                <a-button 
                  v-if="hashOutput" 
                  type="text" 
                  size="small"
                  @click="copyToClipboard(hashOutput)"
                >
                  <template #icon>
                    <icon-copy />
                  </template>
                  复制
                </a-button>
              </a-space>
              <a-textarea 
                v-model="hashOutput" 
                readonly 
                placeholder="哈希值将显示在这里..."
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px; font-family: monospace;"
              />
            </div>
          </a-col>
        </a-row>
        
        <a-alert 
          v-if="hashError" 
          type="error" 
          :message="hashError"
          style="margin-top: 16px;"
          show-icon
        />
      </a-card>
    </a-layout-content>
  </a-layout>
</template>

<style scoped>
.app {
  height: 100vh;
  background: linear-gradient(135deg, #f0f2ff 0%, #f6f8fc 100%);
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

.app :deep(.arco-layout-sider) {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  border-right: 1px solid rgba(0, 0, 0, 0.06);
  box-shadow: 2px 0 8px rgba(0, 0, 0, 0.04);
}

.app :deep(.arco-layout-content) {
  background: transparent;
  padding: 24px;
  overflow-y: auto;
}

.logo {
  padding: 24px;
  text-align: center;
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  margin-bottom: 16px;
}

.nav-menu {
  border: none;
  background: transparent;
}

.nav-menu :deep(.arco-menu-item) {
  margin: 4px 12px;
  border-radius: 8px;
  transition: all 0.2s ease;
}

.nav-menu :deep(.arco-menu-item:hover) {
  background: rgba(24, 144, 255, 0.08);
}

.nav-menu :deep(.arco-menu-item.arco-menu-selected) {
  background: rgba(24, 144, 255, 0.12);
  color: #1890ff;
  font-weight: 500;
}

.tool-icon {
  font-size: 16px;
  margin-right: 8px;
}

.tool-panel {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  border-radius: 16px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.08);
  border: 1px solid rgba(255, 255, 255, 0.2);
  margin-bottom: 24px;
}

.tool-panel :deep(.arco-card-header) {
  border-bottom: 1px solid rgba(0, 0, 0, 0.06);
  padding: 20px 24px;
}

.tool-panel :deep(.arco-card-body) {
  padding: 24px;
}

/* 复制成功提示样式 */
.copy-toast {
  position: fixed;
  top: 30px;
  right: 30px;
  z-index: 1000;
}

/* 特殊样式覆盖 */
.regex-inputs {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 16px;
  margin-bottom: 16px;
}

.uuid-output {
  grid-column: 1 / -1;
}

.uuid-result {
  display: flex;
  gap: 12px;
  align-items: center;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .regex-inputs {
    grid-template-columns: 1fr;
  }
  
  .app :deep(.arco-layout-sider) {
    width: 100% !important;
    height: auto;
  }
  
  .app :deep(.arco-layout-content) {
    padding: 16px;
  }
  
  .tool-panel :deep(.arco-card-body) {
    padding: 16px;
  }
}
</style>

<style>
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  overflow: hidden;
}

#app {
  height: 100vh;
}
</style>