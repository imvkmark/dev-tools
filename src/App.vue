<script setup lang="ts">
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

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

// 复制成功提示
const copySuccess = ref(false);
const copySuccessTimer = ref<number | null>(null);

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
function showCopySuccess() {
  copySuccess.value = true;
  if (copySuccessTimer.value) {
    clearTimeout(copySuccessTimer.value);
  }
  copySuccessTimer.value = setTimeout(() => {
    copySuccess.value = false;
  }, 2000);
}

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
  <div class="app">
    <!-- 复制成功提示 -->
    <div v-if="copySuccess" class="copy-toast">
      <div class="toast-content">
        <span class="toast-icon">✅</span>
        <span class="toast-text">复制成功</span>
      </div>
    </div>

    <!-- 侧边栏 -->
    <div class="sidebar">
      <div class="logo">
        <h2>🛠️ 开发工具箱</h2>
      </div>
      <nav class="nav">
        <button 
          v-for="tool in tools" 
          :key="tool.id"
          :class="['nav-item', { active: activeTab === tool.id }]"
          @click="activeTab = tool.id"
        >
          <span class="icon">{{ tool.icon }}</span>
          <span class="name">{{ tool.name }}</span>
        </button>
      </nav>
    </div>

    <!-- 主内容区 -->
    <div class="main-content">
      <!-- JSON格式化工具 -->
      <div v-if="activeTab === 'json'" class="tool-panel">
        <div class="tool-header">
          <h3>JSON格式化工具</h3>
          <div class="tool-actions">
            <button @click="formatJson" class="btn btn-primary">格式化</button>
            <button @click="minifyJson" class="btn btn-secondary">压缩</button>
          </div>
        </div>
        <div class="tool-content">
          <div class="input-section">
            <label>输入JSON:</label>
            <textarea v-model="jsonInput" placeholder="请输入JSON字符串..."></textarea>
          </div>
          <div class="output-section">
            <label>输出结果:</label>
            <textarea v-model="jsonOutput" readonly placeholder="格式化后的JSON将显示在这里..."></textarea>
            <button v-if="jsonOutput" @click="copyToClipboard(jsonOutput)" class="copy-btn">复制</button>
          </div>
        </div>
        <div v-if="jsonError" class="error">{{ jsonError }}</div>
      </div>

      <!-- Base64编解码工具 -->
      <div v-if="activeTab === 'base64'" class="tool-panel">
        <div class="tool-header">
          <h3>Base64编解码工具</h3>
          <div class="tool-actions">
            <select v-model="base64Mode">
              <option value="encode">编码</option>
              <option value="decode">解码</option>
            </select>
            <button @click="processBase64" class="btn btn-primary">转换</button>
          </div>
        </div>
        <div class="tool-content">
          <div class="input-section">
            <label>输入文本:</label>
            <textarea v-model="base64Input" :placeholder="base64Mode === 'encode' ? '请输入要编码的文本...' : '请输入要解码的Base64字符串...'"></textarea>
          </div>
          <div class="output-section">
            <label>输出结果:</label>
            <textarea v-model="base64Output" readonly placeholder="转换结果将显示在这里..."></textarea>
            <button v-if="base64Output" @click="copyToClipboard(base64Output)" class="copy-btn">复制</button>
          </div>
        </div>
        <div v-if="base64Error" class="error">{{ base64Error }}</div>
      </div>

      <!-- URL编解码工具 -->
      <div v-if="activeTab === 'url'" class="tool-panel">
        <div class="tool-header">
          <h3>URL编解码工具</h3>
          <div class="tool-actions">
            <select v-model="urlMode">
              <option value="encode">编码</option>
              <option value="decode">解码</option>
            </select>
            <button @click="processUrl" class="btn btn-primary">转换</button>
          </div>
        </div>
        <div class="tool-content">
          <div class="input-section">
            <label>输入URL:</label>
            <textarea v-model="urlInput" :placeholder="urlMode === 'encode' ? '请输入要编码的URL...' : '请输入要解码的URL...'"></textarea>
          </div>
          <div class="output-section">
            <label>输出结果:</label>
            <textarea v-model="urlOutput" readonly placeholder="转换结果将显示在这里..."></textarea>
            <button v-if="urlOutput" @click="copyToClipboard(urlOutput)" class="copy-btn">复制</button>
          </div>
        </div>
        <div v-if="urlError" class="error">{{ urlError }}</div>
      </div>

      <!-- 时间戳转换工具 -->
      <div v-if="activeTab === 'timestamp'" class="tool-panel">
        <div class="tool-header">
          <h3>时间戳转换工具</h3>
          <div class="tool-actions">
            <select v-model="timestampMode">
              <option value="toDate">时间戳转日期</option>
              <option value="toTimestamp">日期转时间戳</option>
            </select>
            <select v-model="timestampUnit">
              <option value="seconds">秒</option>
              <option value="milliseconds">毫秒</option>
            </select>
            <button @click="processTimestamp" class="btn btn-primary">转换</button>
          </div>
        </div>
        <div class="tool-content">
          <div class="input-section">
            <label>输入:</label>
            <div class="input-with-helper">
              <textarea v-model="timestampInput" :placeholder="timestampMode === 'toDate' ? '请输入时间戳...' : '请输入日期 (YYYY-MM-DD HH:MM:SS)...'"></textarea>
              <div class="helper-buttons">
                <button v-if="timestampMode === 'toDate'" @click="getCurrentTimestamp" class="btn btn-small">当前时间戳</button>
                <button v-if="timestampMode === 'toTimestamp'" @click="getCurrentDate" class="btn btn-small">当前日期</button>
              </div>
            </div>
          </div>
          <div class="output-section">
            <label>输出结果:</label>
            <textarea v-model="timestampOutput" readonly placeholder="转换结果将显示在这里..."></textarea>
            <button v-if="timestampOutput" @click="copyToClipboard(timestampOutput)" class="copy-btn">复制</button>
          </div>
        </div>
        <div v-if="timestampError" class="error">{{ timestampError }}</div>
      </div>

      <!-- 正则表达式工具 -->
      <div v-if="activeTab === 'regex'" class="tool-panel">
        <div class="tool-header">
          <h3>正则表达式测试工具</h3>
          <div class="tool-actions">
            <button @click="testRegex" class="btn btn-primary">测试</button>
          </div>
        </div>
        <div class="tool-content">
          <div class="regex-inputs">
            <div class="regex-pattern">
              <label>正则表达式:</label>
              <input v-model="regexPattern" placeholder="请输入正则表达式..." />
            </div>
            <div class="regex-flags">
              <label>标志:</label>
              <input v-model="regexFlags" placeholder="i, m, s" />
            </div>
          </div>
          <div class="input-section">
            <label>测试文本:</label>
            <textarea v-model="regexText" placeholder="请输入要测试的文本..."></textarea>
          </div>
          <div class="output-section">
            <label>匹配结果:</label>
            <div v-if="regexResult" class="regex-result">
              <div class="result-summary">
                <span>匹配数量: {{ regexResult.count }}</span>
                <span>是否匹配: {{ regexResult.is_match ? '是' : '否' }}</span>
              </div>
              <div v-if="regexResult.matches.length > 0" class="matches">
                <div v-for="(match, index) in regexResult.matches" :key="index" class="match-item">
                  <span class="match-text">{{ match.match }}</span>
                  <span class="match-position">({{ match.start }}-{{ match.end }})</span>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div v-if="regexError" class="error">{{ regexError }}</div>
      </div>

      <!-- UUID生成工具 -->
      <div v-if="activeTab === 'uuid'" class="tool-panel">
        <div class="tool-header">
          <h3>UUID生成工具</h3>
          <div class="tool-actions">
            <button @click="generateUuid" class="btn btn-primary">生成UUID</button>
          </div>
        </div>
        <div class="tool-content">
          <div class="uuid-output">
            <label>生成的UUID:</label>
            <div class="uuid-result">
              <input v-model="generatedUuid" readonly placeholder="点击生成UUID..." />
              <button v-if="generatedUuid" @click="copyToClipboard(generatedUuid)" class="copy-btn">复制</button>
            </div>
          </div>
        </div>
      </div>

      <!-- 哈希计算工具 -->
      <div v-if="activeTab === 'hash'" class="tool-panel">
        <div class="tool-header">
          <h3>哈希计算工具</h3>
          <div class="tool-actions">
            <select v-model="hashAlgorithm">
              <option value="md5">MD5</option>
              <option value="sha256">SHA256</option>
            </select>
            <button @click="calculateHash" class="btn btn-primary">计算</button>
          </div>
        </div>
        <div class="tool-content">
          <div class="input-section">
            <label>输入文本:</label>
            <textarea v-model="hashInput" placeholder="请输入要计算哈希的文本..."></textarea>
          </div>
          <div class="output-section">
            <label>哈希值:</label>
            <textarea v-model="hashOutput" readonly placeholder="哈希值将显示在这里..."></textarea>
            <button v-if="hashOutput" @click="copyToClipboard(hashOutput)" class="copy-btn">复制</button>
          </div>
        </div>
        <div v-if="hashError" class="error">{{ hashError }}</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.app {
  display: flex;
  height: 100vh;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
  position: relative;
  overflow: hidden;
}

.app::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: 
    radial-gradient(circle at 20% 80%, rgba(120, 119, 198, 0.4) 0%, transparent 60%),
    radial-gradient(circle at 80% 20%, rgba(255, 119, 198, 0.4) 0%, transparent 60%),
    radial-gradient(circle at 40% 40%, rgba(120, 219, 255, 0.3) 0%, transparent 60%),
    radial-gradient(circle at 60% 70%, rgba(255, 200, 87, 0.2) 0%, transparent 50%);
  animation: backgroundShift 20s ease-in-out infinite;
  pointer-events: none;
}

.app::after {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background-image: 
    radial-gradient(2px 2px at 20px 30px, rgba(255, 255, 255, 0.1), transparent),
    radial-gradient(2px 2px at 40px 70px, rgba(255, 255, 255, 0.05), transparent),
    radial-gradient(1px 1px at 90px 40px, rgba(255, 255, 255, 0.08), transparent),
    radial-gradient(1px 1px at 130px 80px, rgba(255, 255, 255, 0.06), transparent);
  background-repeat: repeat;
  background-size: 150px 150px;
  animation: sparkle 30s linear infinite;
  pointer-events: none;
}

@keyframes backgroundShift {
  0%, 100% {
    transform: scale(1) rotate(0deg);
    opacity: 1;
  }
  50% {
    transform: scale(1.1) rotate(2deg);
    opacity: 0.8;
  }
}

@keyframes sparkle {
  0% {
    transform: translateX(0) translateY(0);
  }
  100% {
    transform: translateX(-150px) translateY(-150px);
  }
}

/* 复制成功提示 */
.copy-toast {
  position: fixed;
  top: 30px;
  right: 30px;
  z-index: 1000;
  animation: slideInRight 0.3s ease-out, fadeOut 0.3s ease-in 1.7s forwards;
}

.toast-content {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(20px);
  border-radius: 12px;
  padding: 12px 20px;
  display: flex;
  align-items: center;
  gap: 8px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
  border: 1px solid rgba(255, 255, 255, 0.2);
}

.toast-icon {
  font-size: 16px;
}

.toast-text {
  color: #333;
  font-weight: 500;
  font-size: 14px;
}

@keyframes slideInRight {
  from {
    transform: translateX(100%);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}

@keyframes fadeOut {
  from {
    opacity: 1;
  }
  to {
    opacity: 0;
  }
}

/* 侧边栏样式 */
.sidebar {
  width: 300px;
  background: rgba(255, 255, 255, 0.12);
  backdrop-filter: blur(25px);
  border-right: 1px solid rgba(255, 255, 255, 0.25);
  display: flex;
  flex-direction: column;
  box-shadow: 
    4px 0 30px rgba(0, 0, 0, 0.1),
    inset -1px 0 0 rgba(255, 255, 255, 0.2);
  position: relative;
  z-index: 10;
}

.sidebar::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: linear-gradient(180deg, 
    rgba(255, 255, 255, 0.18) 0%, 
    rgba(255, 255, 255, 0.08) 50%, 
    rgba(255, 255, 255, 0.12) 100%);
  pointer-events: none;
}

.sidebar::after {
  content: '';
  position: absolute;
  top: 20px;
  left: 20px;
  right: 20px;
  height: 1px;
  background: linear-gradient(90deg, 
    transparent 0%, 
    rgba(255, 255, 255, 0.4) 50%, 
    transparent 100%);
}

.logo {
  padding: 32px 24px;
  border-bottom: 1px solid rgba(255, 255, 255, 0.15);
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  text-align: center;
  position: relative;
  margin-bottom: 24px;
}

.logo::after {
  content: '';
  position: absolute;
  bottom: -1px;
  left: 50%;
  transform: translateX(-50%);
  width: 60px;
  height: 2px;
  background: linear-gradient(90deg, #4facfe 0%, #00f2fe 100%);
  border-radius: 1px;
}

.logo h2 {
  margin: 0;
  font-size: 22px;
  font-weight: 700;
  text-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
  position: relative;
  display: inline-block;
}

.logo h2::before {
  content: '';
  position: absolute;
  top: -5px;
  left: -5px;
  right: -5px;
  bottom: -5px;
  background: linear-gradient(45deg, 
    rgba(255, 255, 255, 0.1) 0%, 
    transparent 50%, 
    rgba(255, 255, 255, 0.1) 100%);
  border-radius: 12px;
  z-index: -1;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.logo:hover h2::before {
  opacity: 1;
}

.tool-header {
  display: flex;
  justify-content: space-between;
  align-items: flex-start;
  margin-bottom: 32px;
  padding-bottom: 20px;
  border-bottom: 1px solid rgba(0, 0, 0, 0.08);
  position: relative;
}

.tool-header::after {
  content: '';
  position: absolute;
  bottom: -1px;
  left: 0;
  width: 80px;
  height: 2px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border-radius: 1px;
}

.tool-header h3 {
  margin: 0;
  color: #2d3748;
  font-size: 32px;
  font-weight: 700;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  position: relative;
  line-height: 1.2;
}

.tool-actions {
  display: flex;
  gap: 12px;
  align-items: center;
  flex-wrap: wrap;
}

.tool-actions select {
  min-width: 120px;
  margin-bottom: 0;
}

.nav {
  flex: 1;
  padding: 20px 0;
}

.nav-item {
  display: flex;
  align-items: center;
  width: 100%;
  padding: 18px 28px;
  border: none;
  background: none;
  text-align: left;
  cursor: pointer;
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  color: rgba(255, 255, 255, 0.9);
  font-size: 15px;
  border-radius: 0;
  position: relative;
  margin: 3px 12px;
  border-left: 3px solid transparent;
  backdrop-filter: blur(10px);
  overflow: hidden;
}

.nav-item::before {
  content: '';
  position: absolute;
  left: 0;
  top: 0;
  bottom: 0;
  width: 3px;
  background: linear-gradient(180deg, #4facfe 0%, #00f2fe 100%);
  border-radius: 0 2px 2px 0;
  transform: scaleY(0);
  transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.nav-item:hover {
  background: rgba(102, 126, 234, 0.15);
  color: #667eea;
  transform: translateX(4px);
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
  border-radius: 0 16px 16px 0;
}

.nav-item:hover .icon {
  transform: scale(1.1) rotate(5deg);
}

.nav-item:hover::before {
  transform: scaleY(1);
}

.nav-item.active {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  box-shadow: 0 6px 25px rgba(102, 126, 234, 0.4);
  transform: translateX(6px);
  border-radius: 0 16px 16px 0;
}

.nav-item.active .icon {
  transform: scale(1.15);
  filter: drop-shadow(0 2px 8px rgba(0, 0, 0, 0.3));
}

.nav-item.active::before {
  transform: scaleY(1);
}

.nav-item .icon {
  margin-right: 14px;
  font-size: 20px;
  width: 24px;
  text-align: center;
  transition: transform 0.3s ease;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.2));
}

.nav-item .name {
  font-weight: 500;
  letter-spacing: 0.3px;
}

/* 主内容区样式 */
.main-content {
  flex: 1;
  padding: 40px;
  overflow-y: auto;
  position: relative;
  z-index: 1;
}

.tool-panel {
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(25px);
  border-radius: 28px;
  padding: 36px;
  box-shadow: 
    0 25px 80px rgba(0, 0, 0, 0.12),
    0 12px 35px rgba(0, 0, 0, 0.08),
    inset 0 1px 0 rgba(255, 255, 255, 0.7),
    inset 0 -1px 0 rgba(0, 0, 0, 0.05);
  border: 1px solid rgba(255, 255, 255, 0.4);
  position: relative;
  overflow: hidden;
  animation: panelFadeIn 0.6s ease-out;
}

.tool-panel::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(90deg, 
    transparent 0%, 
    rgba(102, 126, 234, 0.6) 20%,
    rgba(118, 75, 162, 0.6) 80%,
    transparent 100%);
  border-radius: 28px 28px 0 0;
}

.tool-panel::after {
  content: '';
  position: absolute;
  top: 50%;
  left: 50%;
  width: 200%;
  height: 200%;
  background: radial-gradient(circle, 
    rgba(255, 255, 255, 0.1) 0%, 
    transparent 70%);
  transform: translate(-50%, -50%);
  pointer-events: none;
  opacity: 0;
  transition: opacity 0.3s ease;
}

.tool-panel:hover::after {
  opacity: 1;
}

@keyframes panelFadeIn {
  from {
    opacity: 0;
    transform: translateY(20px) scale(0.98);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.tool-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 30px;
  padding-bottom: 20px;
  border-bottom: 2px solid rgba(102, 126, 234, 0.1);
}

.tool-header h3 {
  margin: 0;
  color: #333;
  font-size: 24px;
  font-weight: 600;
}

.tool-actions {
  display: flex;
  gap: 10px;
  align-items: center;
}

.tool-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 24px;
  margin-top: 24px;
}

.input-section,
.output-section {
  background: rgba(248, 250, 252, 0.8);
  backdrop-filter: blur(10px);
  border-radius: 20px;
  padding: 24px;
  border: 1px solid rgba(226, 232, 240, 0.6);
  display: flex;
  flex-direction: column;
  margin-bottom: 28px;
  position: relative;
  transition: all 0.3s ease;
}

.input-section:hover,
.output-section:hover {
  background: rgba(248, 250, 252, 0.95);
  border-color: rgba(102, 126, 234, 0.3);
  box-shadow: 0 8px 25px rgba(102, 126, 234, 0.1);
}

.input-section label,
.output-section label {
  margin: 0 0 16px 0;
  color: #2d3748;
  font-size: 14px;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 10px;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

.input-section label::before {
  content: '📝';
  font-size: 16px;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
}

.output-section label::before {
  content: '📋';
  font-size: 16px;
  filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
}

.input-section textarea,
.output-section textarea {
  min-height: 220px;
  padding: 18px 20px;
  border: 2px solid #e2e8f0;
  border-radius: 16px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 14px;
  line-height: 1.6;
  resize: vertical;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  background: rgba(255, 255, 255, 0.9);
  backdrop-filter: blur(10px);
  box-shadow: 
    0 4px 12px rgba(0, 0, 0, 0.05),
    inset 0 1px 0 rgba(255, 255, 255, 0.8);
}

.input-section textarea:focus,
.output-section textarea:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 
    0 0 0 4px rgba(102, 126, 234, 0.1),
    0 8px 25px rgba(102, 126, 234, 0.15),
    inset 0 1px 0 rgba(255, 255, 255, 0.8);
  transform: translateY(-1px);
}

.input-section textarea:hover,
.output-section textarea:hover {
  border-color: #cbd5e0;
  box-shadow: 
    0 6px 20px rgba(0, 0, 0, 0.08),
    inset 0 1px 0 rgba(255, 255, 255, 0.8);
}

.output-section {
  position: relative;
}

.copy-btn {
  position: absolute;
  top: 35px;
  right: 10px;
  padding: 10px 16px;
  background: linear-gradient(135deg, #48bb78 0%, #38a169 100%);
  color: white;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  font-size: 12px;
  font-weight: 600;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 
    0 4px 15px rgba(72, 187, 120, 0.3),
    inset 0 1px 0 rgba(255, 255, 255, 0.2);
  position: relative;
  overflow: hidden;
}

.copy-btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, 
    transparent 0%, 
    rgba(255, 255, 255, 0.2) 50%, 
    transparent 100%);
  transition: left 0.6s ease;
}

.copy-btn:hover {
  background: linear-gradient(135deg, #68d391 0%, #48bb78 100%);
  transform: translateY(-2px);
  box-shadow: 
    0 8px 25px rgba(72, 187, 120, 0.4),
    inset 0 1px 0 rgba(255, 255, 255, 0.3);
}

.copy-btn:hover::before {
  left: 100%;
}

.copy-btn:active {
  transform: translateY(0);
  box-shadow: 
    0 4px 15px rgba(72, 187, 120, 0.3),
    inset 0 1px 0 rgba(255, 255, 255, 0.2);
}

/* 按钮样式 */
.btn {
  padding: 14px 28px;
  border: none;
  border-radius: 14px;
  cursor: pointer;
  font-weight: 600;
  font-size: 14px;
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
  display: inline-flex;
  align-items: center;
  gap: 8px;
  position: relative;
  overflow: hidden;
  margin-right: 12px;
  margin-bottom: 12px;
}

.btn::before {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, 
    transparent 0%, 
    rgba(255, 255, 255, 0.2) 50%, 
    transparent 100%);
  transition: left 0.6s ease;
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  box-shadow: 
    0 4px 15px rgba(102, 126, 234, 0.3),
    inset 0 1px 0 rgba(255, 255, 255, 0.2);
}

.btn-primary:hover {
  transform: translateY(-3px);
  box-shadow: 
    0 12px 35px rgba(102, 126, 234, 0.4),
    inset 0 1px 0 rgba(255, 255, 255, 0.3);
  background: linear-gradient(135deg, #7c8cfa 0%, #8a5cb8 100%);
}

.btn-primary:hover::before {
  left: 100%;
}

.btn-primary:active {
  transform: translateY(-1px);
  box-shadow: 
    0 6px 20px rgba(102, 126, 234, 0.3),
    inset 0 1px 0 rgba(255, 255, 255, 0.2);
}

.btn-secondary {
  background: linear-gradient(135deg, #f7fafc 0%, #edf2f7 100%);
  color: #4a5568;
  border: 2px solid #e2e8f0;
  box-shadow: 
    0 4px 15px rgba(0, 0, 0, 0.05),
    inset 0 1px 0 rgba(255, 255, 255, 0.8);
}

.btn-secondary:hover {
  background: linear-gradient(135deg, #edf2f7 0%, #e2e8f0 100%);
  border-color: #cbd5e0;
  transform: translateY(-2px);
  box-shadow: 
    0 8px 25px rgba(0, 0, 0, 0.1),
    inset 0 1px 0 rgba(255, 255, 255, 0.9);
}

.btn-small {
  padding: 8px 16px;
  font-size: 12px;
  border-radius: 10px;
}

/* 选择框样式 */
select {
  padding: 12px 16px;
  border: 2px solid rgba(226, 232, 240, 0.8);
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(15px);
  color: #2d3748;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 
    0 6px 20px rgba(0, 0, 0, 0.06),
    inset 0 1px 0 rgba(255, 255, 255, 0.9);
  appearance: none;
  background-image: url("data:image/svg+xml,%3csvg xmlns='http://www.w3.org/2000/svg' fill='none' viewBox='0 0 20 20'%3e%3cpath stroke='%236b7280' stroke-linecap='round' stroke-linejoin='round' stroke-width='1.5' d='m6 8 4 4 4-4'/%3e%3c/svg%3e");
  background-position: right 12px center;
  background-repeat: no-repeat;
  background-size: 16px;
  padding-right: 40px;
}

select:hover {
  border-color: rgba(102, 126, 234, 0.4);
  box-shadow: 
    0 8px 25px rgba(102, 126, 234, 0.1),
    inset 0 1px 0 rgba(255, 255, 255, 0.9);
  transform: translateY(-1px);
}

select:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 
    0 0 0 4px rgba(102, 126, 234, 0.15),
    0 12px 30px rgba(102, 126, 234, 0.2),
    inset 0 1px 0 rgba(255, 255, 255, 0.9);
  transform: translateY(-2px);
}

/* 错误提示样式 */
.error {
  color: #c53030;
  font-size: 14px;
  margin-top: 12px;
  padding: 14px 18px;
  background: linear-gradient(135deg, rgba(254, 178, 178, 0.15) 0%, rgba(252, 165, 165, 0.1) 100%);
  border-radius: 12px;
  border: 1px solid rgba(239, 68, 68, 0.2);
  border-left: 4px solid #e53e3e;
  backdrop-filter: blur(10px);
  box-shadow: 
    0 4px 12px rgba(239, 68, 68, 0.1),
    inset 0 1px 0 rgba(255, 255, 255, 0.5);
  display: flex;
  align-items: center;
  gap: 10px;
  animation: slideInDown 0.3s ease-out;
}

.error::before {
  content: '⚠️';
  font-size: 16px;
  flex-shrink: 0;
}

@keyframes slideInDown {
  from {
    transform: translateY(-10px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

/* 正则表达式特殊样式 */
.regex-inputs {
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 15px;
  margin-bottom: 20px;
}

.regex-pattern,
.regex-flags {
  display: flex;
  flex-direction: column;
}

.regex-pattern input,
.regex-flags input {
  padding: 14px 16px;
  border: 2px solid rgba(226, 232, 240, 0.8);
  border-radius: 12px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 14px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(15px);
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 
    0 6px 20px rgba(0, 0, 0, 0.06),
    inset 0 1px 0 rgba(255, 255, 255, 0.9);
  color: #2d3748;
}

.regex-pattern input:focus,
.regex-flags input:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 
    0 0 0 4px rgba(102, 126, 234, 0.15),
    0 12px 30px rgba(102, 126, 234, 0.2),
    inset 0 1px 0 rgba(255, 255, 255, 0.9);
  transform: translateY(-1px);
}

.regex-pattern input:hover,
.regex-flags input:hover {
  border-color: rgba(102, 126, 234, 0.4);
  box-shadow: 
    0 8px 25px rgba(102, 126, 234, 0.1),
    inset 0 1px 0 rgba(255, 255, 255, 0.9);
}

.regex-result {
  background: #f7fafc;
  border-radius: 8px;
  padding: 15px;
  min-height: 200px;
}

.result-summary {
  display: flex;
  gap: 20px;
  margin-bottom: 15px;
  font-weight: 600;
  color: #4a5568;
}

.matches {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.match-item {
  background: white;
  padding: 8px 12px;
  border-radius: 6px;
  border-left: 3px solid #667eea;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.match-text {
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  background: #edf2f7;
  padding: 2px 6px;
  border-radius: 4px;
  font-size: 13px;
}

.match-position {
  color: #718096;
  font-size: 12px;
}

/* UUID工具特殊样式 */
.uuid-output {
  grid-column: 1 / -1;
}

.uuid-result {
  display: flex;
  gap: 10px;
  align-items: center;
}

.uuid-result input {
  flex: 1;
  padding: 18px 20px;
  border: 2px solid rgba(226, 232, 240, 0.8);
  border-radius: 16px;
  font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;
  font-size: 14px;
  background: rgba(255, 255, 255, 0.95);
  backdrop-filter: blur(15px);
  transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
  box-shadow: 
    0 6px 20px rgba(0, 0, 0, 0.06),
    inset 0 1px 0 rgba(255, 255, 255, 0.9);
  color: #2d3748;
  font-weight: 500;
}

.uuid-result input:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 
    0 0 0 4px rgba(102, 126, 234, 0.15),
    0 12px 30px rgba(102, 126, 234, 0.2),
    inset 0 1px 0 rgba(255, 255, 255, 0.9);
  transform: translateY(-1px);
}

.uuid-result input:hover {
  border-color: rgba(102, 126, 234, 0.4);
  box-shadow: 
    0 8px 25px rgba(102, 126, 234, 0.1),
    inset 0 1px 0 rgba(255, 255, 255, 0.9);
}

/* 时间戳工具特殊样式 */
.input-with-helper {
  position: relative;
}

.helper-buttons {
  position: absolute;
  top: 35px;
  right: 10px;
}

/* 响应式设计 */
@media (max-width: 768px) {
  .app {
    flex-direction: column;
  }
  
  .sidebar {
    width: 100%;
    height: auto;
  }
  
  .nav {
    display: flex;
    overflow-x: auto;
    padding: 10px;
  }
  
  .nav-item {
    min-width: 120px;
    text-align: center;
  }
  
  .tool-content {
    grid-template-columns: 1fr;
    gap: 20px;
  }
  
  .regex-inputs {
    grid-template-columns: 1fr;
  }
}

/* 页面加载动画 */
@keyframes fadeInUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.nav-item {
  animation: fadeInUp 0.6s ease-out;
  animation-fill-mode: both;
}

.nav-item:nth-child(1) { animation-delay: 0.1s; }
.nav-item:nth-child(2) { animation-delay: 0.2s; }
.nav-item:nth-child(3) { animation-delay: 0.3s; }
.nav-item:nth-child(4) { animation-delay: 0.4s; }
.nav-item:nth-child(5) { animation-delay: 0.5s; }
.nav-item:nth-child(6) { animation-delay: 0.6s; }
.nav-item:nth-child(7) { animation-delay: 0.7s; }

/* 滚动条样式 */
::-webkit-scrollbar {
  width: 10px;
}

::-webkit-scrollbar-track {
  background: rgba(255, 255, 255, 0.1);
  border-radius: 10px;
  margin: 5px;
}

::-webkit-scrollbar-thumb {
  background: linear-gradient(180deg, 
    rgba(255, 255, 255, 0.4) 0%, 
    rgba(255, 255, 255, 0.2) 100%);
  border-radius: 10px;
  border: 2px solid transparent;
  background-clip: content-box;
  transition: all 0.3s ease;
}

::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(180deg, 
    rgba(255, 255, 255, 0.6) 0%, 
    rgba(255, 255, 255, 0.4) 100%);
  background-clip: content-box;
}

/* 工具内容区域滚动条 */
.main-content::-webkit-scrollbar-track {
  background: rgba(0, 0, 0, 0.05);
  border-radius: 10px;
}

.main-content::-webkit-scrollbar-thumb {
  background: linear-gradient(180deg, 
    rgba(102, 126, 234, 0.3) 0%, 
    rgba(118, 75, 162, 0.3) 100%);
  border-radius: 10px;
}

.main-content::-webkit-scrollbar-thumb:hover {
  background: linear-gradient(180deg, 
    rgba(102, 126, 234, 0.5) 0%, 
    rgba(118, 75, 162, 0.5) 100%);
}

/* 响应式设计 */
@media (max-width: 1200px) {
  .sidebar {
    width: 260px;
  }
  
  .tool-content {
    grid-template-columns: 1fr;
    gap: 20px;
  }
  
  .tool-panel {
    padding: 28px;
  }
}

@media (max-width: 768px) {
  .app {
    flex-direction: column;
  }
  
  .sidebar {
    width: 100%;
    height: auto;
    padding: 20px 0;
  }
  
  .nav {
    display: flex;
    overflow-x: auto;
    padding: 0 20px;
    gap: 8px;
  }
  
  .nav-item {
    white-space: nowrap;
    margin: 0;
    border-radius: 12px;
    min-width: auto;
  }
  
  .main-content {
    padding: 20px;
  }
  
  .tool-panel {
    padding: 20px;
    border-radius: 20px;
  }
  
  .tool-header {
    flex-direction: column;
    align-items: flex-start;
    gap: 16px;
  }
  
  .tool-actions {
    width: 100%;
    justify-content: flex-start;
  }
}

/* 高分辨率屏幕优化 */
@media (-webkit-min-device-pixel-ratio: 2), (min-resolution: 192dpi) {
  .tool-panel {
    border-width: 0.5px;
  }
  
  .sidebar {
    border-right-width: 0.5px;
  }
}

/* 深色模式支持 */
@media (prefers-color-scheme: dark) {
  .tool-panel {
    background: rgba(26, 32, 44, 0.95);
    border-color: rgba(255, 255, 255, 0.1);
  }
  
  .input-section, .output-section {
    background: rgba(45, 55, 72, 0.8);
    border-color: rgba(255, 255, 255, 0.1);
  }
  
  .input-section label, .output-section label {
    color: #e2e8f0;
  }
  
  textarea, input, select {
    background: rgba(45, 55, 72, 0.9);
    border-color: rgba(255, 255, 255, 0.2);
    color: #e2e8f0;
  }
}

/* 动画性能优化 */
@media (prefers-reduced-motion: reduce) {
  * {
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
    transition-duration: 0.01ms !important;
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