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

// 二维码工具
const qrInput = ref('');
const qrOutput = ref('');
const qrMode = ref('generate'); // generate | decode
const qrError = ref('');
const qrColor = ref('#000000');
const qrBgColor = ref('#ffffff');
const qrSize = ref(256);

// 代码美化工具
const codeFormatInput = ref('');
const codeFormatOutput = ref('');
const codeFormatType = ref('javascript'); // javascript | css | html | xml | sql
const codeFormatError = ref('');

// 代码压缩工具
const codeCompressInput = ref('');
const codeCompressOutput = ref('');
const codeCompressType = ref('javascript'); // javascript | css | html
const codeCompressError = ref('');

// 字符串编码工具
const stringEncodeInput = ref('');
const stringEncodeOutput = ref('');
const stringEncodeType = ref('unicode'); // unicode | utf8 | md5 | sha1 | sha256
const stringEncodeMode = ref('encode'); // encode | decode
const stringEncodeError = ref('');

// 密码生成工具
const passwordLength = ref(16);
const passwordIncludeUppercase = ref(true);
const passwordIncludeLowercase = ref(true);
const passwordIncludeNumbers = ref(true);
const passwordIncludeSymbols = ref(false);
const generatedPassword = ref('');

// 颜色工具
const colorInput = ref('#1890ff');
const colorOutput = ref('');
const colorFormat = ref('hex'); // hex | rgb | hsl | hsv
const colorError = ref('');

// Markdown工具
const markdownInput = ref('');
const markdownOutput = ref('');
const markdownMode = ref('preview'); // preview | html

// API测试工具
const apiUrl = ref('');
const apiMethod = ref('GET');
const apiHeaders = ref('{}');
const apiBody = ref('');
const apiResponse = ref('');
const apiError = ref('');
const apiLoading = ref(false);

// 便签笔记工具
const noteTitle = ref('');
const noteContent = ref('');
const notesList = ref<Array<{id: string, title: string, content: string, createdAt: string}>>([]);
const selectedNoteId = ref('');

// 图片工具
const imageFile = ref<File | null>(null);
const imagePreview = ref('');
const imageFormat = ref('png'); // png | jpg | webp
const imageQuality = ref(0.8);
const imageError = ref('');

// 工具列表
const tools = [
  // 原有工具
  { id: 'json', name: 'JSON格式化', icon: '{}' },
  { id: 'base64', name: 'Base64编解码', icon: '🔐' },
  { id: 'url', name: 'URL编解码', icon: '🔗' },
  { id: 'timestamp', name: '时间戳转换', icon: '⏰' },
  { id: 'regex', name: '正则表达式', icon: '📝' },
  { id: 'uuid', name: 'UUID生成', icon: '🆔' },
  { id: 'hash', name: '哈希计算', icon: '#️⃣' },
  
  // FeHelper 新增工具
  { id: 'qrcode', name: '二维码工具', icon: '📱' },
  { id: 'code-format', name: '代码美化', icon: '✨' },
  { id: 'code-compress', name: '代码压缩', icon: '📦' },
  { id: 'string-encode', name: '字符串编码', icon: '🔤' },
  { id: 'password-gen', name: '密码生成', icon: '🔑' },
  { id: 'color-tools', name: '颜色工具', icon: '🎨' },
  { id: 'markdown', name: 'Markdown', icon: '📄' },
  { id: 'api-test', name: 'API测试', icon: '🌐' },
  { id: 'notes', name: '便签笔记', icon: '📝' },
  { id: 'image-tools', name: '图片工具', icon: '🖼️' }
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

// 二维码生成
async function generateQRCode() {
  try {
    qrError.value = '';
    const QRCode = await import('qrcode');
    qrOutput.value = await QRCode.toDataURL(qrInput.value, {
      width: qrSize.value,
      color: {
        dark: qrColor.value,
        light: qrBgColor.value
      }
    });
  } catch (error) {
    qrError.value = '二维码生成失败: ' + (error as Error).message;
    qrOutput.value = '';
  }
}

// 代码美化
async function formatCode() {
  try {
    codeFormatError.value = '';
    const beautify = await import('js-beautify');
    
    switch (codeFormatType.value) {
      case 'javascript':
        codeFormatOutput.value = beautify.js(codeFormatInput.value, {
          indent_size: 2,
          space_in_empty_paren: true
        });
        break;
      case 'css':
        codeFormatOutput.value = beautify.css(codeFormatInput.value, {
          indent_size: 2
        });
        break;
      case 'html':
      case 'xml':
        codeFormatOutput.value = beautify.html(codeFormatInput.value, {
          indent_size: 2,
          wrap_line_length: 120
        });
        break;
      case 'sql':
        // 简单的SQL格式化
        codeFormatOutput.value = codeFormatInput.value
          .replace(/\s+/g, ' ')
          .replace(/,/g, ',\n  ')
          .replace(/\bFROM\b/gi, '\nFROM')
          .replace(/\bWHERE\b/gi, '\nWHERE')
          .replace(/\bAND\b/gi, '\n  AND')
          .replace(/\bOR\b/gi, '\n  OR')
          .replace(/\bORDER BY\b/gi, '\nORDER BY')
          .replace(/\bGROUP BY\b/gi, '\nGROUP BY')
          .trim();
        break;
      default:
        codeFormatOutput.value = codeFormatInput.value;
    }
  } catch (error) {
    codeFormatError.value = '代码格式化失败: ' + (error as Error).message;
    codeFormatOutput.value = '';
  }
}

// 代码压缩
async function compressCode() {
  try {
    codeCompressError.value = '';
    
    switch (codeCompressType.value) {
      case 'javascript':
        const UglifyJS = await import('uglify-js');
        const result = UglifyJS.minify(codeCompressInput.value);
        if (result.error) {
          throw new Error(result.error.message);
        }
        codeCompressOutput.value = result.code || '';
        break;
      case 'css':
        const CleanCSS = await import('clean-css');
        const cleanCSS = new CleanCSS.default();
        const cssResult = cleanCSS.minify(codeCompressInput.value);
        if (cssResult.errors.length > 0) {
          throw new Error(cssResult.errors.join(', '));
        }
        codeCompressOutput.value = cssResult.styles;
        break;
      case 'html':
        const htmlMinifier = await import('html-minifier');
        codeCompressOutput.value = htmlMinifier.minify(codeCompressInput.value, {
          removeComments: true,
          removeRedundantAttributes: true,
          removeScriptTypeAttributes: true,
          removeStyleLinkTypeAttributes: true,
          collapseWhitespace: true,
          minifyCSS: true,
          minifyJS: true
        });
        break;
      default:
        codeCompressOutput.value = codeCompressInput.value;
    }
  } catch (error) {
    codeCompressError.value = '代码压缩失败: ' + (error as Error).message;
    codeCompressOutput.value = '';
  }
}

// 字符串编码
async function processStringEncode() {
  try {
    stringEncodeError.value = '';
    const CryptoJS = await import('crypto-js');
    
    switch (stringEncodeType.value) {
      case 'unicode':
        if (stringEncodeMode.value === 'encode') {
          stringEncodeOutput.value = stringEncodeInput.value
            .split('')
            .map(char => '\\u' + char.charCodeAt(0).toString(16).padStart(4, '0'))
            .join('');
        } else {
          stringEncodeOutput.value = stringEncodeInput.value
            .replace(/\\u[\dA-Fa-f]{4}/g, match => 
              String.fromCharCode(parseInt(match.replace('\\u', ''), 16))
            );
        }
        break;
      case 'utf8':
        if (stringEncodeMode.value === 'encode') {
          stringEncodeOutput.value = encodeURIComponent(stringEncodeInput.value);
        } else {
          stringEncodeOutput.value = decodeURIComponent(stringEncodeInput.value);
        }
        break;
      case 'md5':
        stringEncodeOutput.value = CryptoJS.MD5(stringEncodeInput.value).toString();
        break;
      case 'sha1':
        stringEncodeOutput.value = CryptoJS.SHA1(stringEncodeInput.value).toString();
        break;
      case 'sha256':
        stringEncodeOutput.value = CryptoJS.SHA256(stringEncodeInput.value).toString();
        break;
      default:
        stringEncodeOutput.value = stringEncodeInput.value;
    }
  } catch (error) {
    stringEncodeError.value = '编码处理失败: ' + (error as Error).message;
    stringEncodeOutput.value = '';
  }
}

// 生成随机密码
function generatePassword() {
  const uppercase = 'ABCDEFGHIJKLMNOPQRSTUVWXYZ';
  const lowercase = 'abcdefghijklmnopqrstuvwxyz';
  const numbers = '0123456789';
  const symbols = '!@#$%^&*()_+-=[]{}|;:,.<>?';
  
  let charset = '';
  if (passwordIncludeUppercase.value) charset += uppercase;
  if (passwordIncludeLowercase.value) charset += lowercase;
  if (passwordIncludeNumbers.value) charset += numbers;
  if (passwordIncludeSymbols.value) charset += symbols;
  
  if (charset === '') {
    charset = lowercase; // 默认至少包含小写字母
  }
  
  let password = '';
  for (let i = 0; i < passwordLength.value; i++) {
    password += charset.charAt(Math.floor(Math.random() * charset.length));
  }
  
  generatedPassword.value = password;
}

// 颜色转换
function convertColor() {
  try {
    colorError.value = '';
    const color = colorInput.value;
    
    // 解析颜色值
    let r, g, b;
    if (color.startsWith('#')) {
      const hex = color.slice(1);
      r = parseInt(hex.slice(0, 2), 16);
      g = parseInt(hex.slice(2, 4), 16);
      b = parseInt(hex.slice(4, 6), 16);
    } else {
      throw new Error('请输入有效的十六进制颜色值');
    }
    
    switch (colorFormat.value) {
      case 'hex':
        colorOutput.value = color.toUpperCase();
        break;
      case 'rgb':
        colorOutput.value = `rgb(${r}, ${g}, ${b})`;
        break;
      case 'hsl':
        const hsl = rgbToHsl(r, g, b);
        colorOutput.value = `hsl(${Math.round(hsl.h)}, ${Math.round(hsl.s)}%, ${Math.round(hsl.l)}%)`;
        break;
      case 'hsv':
        const hsv = rgbToHsv(r, g, b);
        colorOutput.value = `hsv(${Math.round(hsv.h)}, ${Math.round(hsv.s)}%, ${Math.round(hsv.v)}%)`;
        break;
    }
  } catch (error) {
    colorError.value = '颜色转换失败: ' + (error as Error).message;
    colorOutput.value = '';
  }
}

// RGB转HSL
function rgbToHsl(r: number, g: number, b: number) {
  r /= 255;
  g /= 255;
  b /= 255;
  
  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  let h = 0, s = 0, l = (max + min) / 2;
  
  if (max !== min) {
    const d = max - min;
    s = l > 0.5 ? d / (2 - max - min) : d / (max + min);
    
    switch (max) {
      case r: h = (g - b) / d + (g < b ? 6 : 0); break;
      case g: h = (b - r) / d + 2; break;
      case b: h = (r - g) / d + 4; break;
    }
    h /= 6;
  }
  
  return { h: h * 360, s: s * 100, l: l * 100 };
}

// RGB转HSV
function rgbToHsv(r: number, g: number, b: number) {
  r /= 255;
  g /= 255;
  b /= 255;
  
  const max = Math.max(r, g, b);
  const min = Math.min(r, g, b);
  let h = 0, s = 0, v = max;
  
  const d = max - min;
  s = max === 0 ? 0 : d / max;
  
  if (max !== min) {
    switch (max) {
      case r: h = (g - b) / d + (g < b ? 6 : 0); break;
      case g: h = (b - r) / d + 2; break;
      case b: h = (r - g) / d + 4; break;
    }
    h /= 6;
  }
  
  return { h: h * 360, s: s * 100, v: v * 100 };
}

// Markdown处理
async function processMarkdown() {
  try {
    const marked = await import('marked');
    if (markdownMode.value === 'preview') {
      markdownOutput.value = await marked.parse(markdownInput.value);
    } else {
      markdownOutput.value = await marked.parse(markdownInput.value);
    }
  } catch (error) {
    console.error('Markdown处理失败:', error);
  }
}

// API测试
async function testAPI() {
  try {
    apiError.value = '';
    apiLoading.value = true;
    
    const headers: Record<string, string> = {};
    try {
      const parsedHeaders = JSON.parse(apiHeaders.value);
      Object.assign(headers, parsedHeaders);
    } catch {
      // 忽略JSON解析错误，使用默认headers
    }
    
    const options: RequestInit = {
      method: apiMethod.value,
      headers: {
        'Content-Type': 'application/json',
        ...headers
      }
    };
    
    if (apiMethod.value !== 'GET' && apiBody.value) {
      options.body = apiBody.value;
    }
    
    const response = await fetch(apiUrl.value, options);
    const responseText = await response.text();
    
    apiResponse.value = JSON.stringify({
      status: response.status,
      statusText: response.statusText,
      headers: Object.fromEntries(response.headers.entries()),
      body: responseText
    }, null, 2);
  } catch (error) {
    apiError.value = 'API请求失败: ' + (error as Error).message;
    apiResponse.value = '';
  } finally {
    apiLoading.value = false;
  }
}

// 便签笔记功能
function saveNote() {
  if (!noteTitle.value.trim() || !noteContent.value.trim()) {
    return;
  }
  
  const note = {
    id: Date.now().toString(),
    title: noteTitle.value,
    content: noteContent.value,
    createdAt: new Date().toLocaleString()
  };
  
  notesList.value.unshift(note);
  noteTitle.value = '';
  noteContent.value = '';
  
  // 保存到localStorage
  localStorage.setItem('dev-tools-notes', JSON.stringify(notesList.value));
}

function loadNote(noteId: string) {
  const note = notesList.value.find(n => n.id === noteId);
  if (note) {
    noteTitle.value = note.title;
    noteContent.value = note.content;
    selectedNoteId.value = noteId;
  }
}

function deleteNote(noteId: string) {
  notesList.value = notesList.value.filter(n => n.id !== noteId);
  if (selectedNoteId.value === noteId) {
    noteTitle.value = '';
    noteContent.value = '';
    selectedNoteId.value = '';
  }
  localStorage.setItem('dev-tools-notes', JSON.stringify(notesList.value));
}

// 加载保存的笔记
function loadSavedNotes() {
  try {
    const saved = localStorage.getItem('dev-tools-notes');
    if (saved) {
      notesList.value = JSON.parse(saved);
    }
  } catch (error) {
    console.error('加载笔记失败:', error);
  }
}

// 初始化时加载笔记
loadSavedNotes();

// 图片处理功能
function handleImageUpload(event: Event) {
  const target = event.target as HTMLInputElement;
  const file = target.files?.[0];
  
  if (file) {
    imageFile.value = file;
    const reader = new FileReader();
    reader.onload = (e) => {
      imagePreview.value = e.target?.result as string;
    };
    reader.readAsDataURL(file);
  }
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

      <!-- 二维码工具 -->
      <a-card v-if="activeTab === 'qrcode'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            二维码工具
          </a-typography-title>
        </template>
        <template #extra>
          <a-space>
            <a-select v-model="qrMode" style="width: 120px;">
              <a-option value="generate">生成</a-option>
              <a-option value="decode">解码</a-option>
            </a-select>
            <a-button type="primary" @click="generateQRCode" v-if="qrMode === 'generate'">生成</a-button>
          </a-space>
        </template>
        
        <a-row :gutter="24">
          <a-col :span="12">
            <div class="input-section">
              <a-typography-text strong>输入内容:</a-typography-text>
              <a-textarea 
                v-model="qrInput" 
                placeholder="请输入要生成二维码的内容..."
                :rows="8"
                :auto-size="{ minRows: 8, maxRows: 12 }"
                style="margin-top: 8px;"
              />
              
              <div style="margin-top: 16px;" v-if="qrMode === 'generate'">
                <a-space direction="vertical" style="width: 100%;" :size="12">
                  <div>
                    <a-typography-text strong>前景色:</a-typography-text>
                    <input type="color" v-model="qrColor" style="margin-left: 8px; width: 40px; height: 32px; border: none; border-radius: 4px;">
                  </div>
                  <div>
                    <a-typography-text strong>背景色:</a-typography-text>
                    <input type="color" v-model="qrBgColor" style="margin-left: 8px; width: 40px; height: 32px; border: none; border-radius: 4px;">
                  </div>
                  <div>
                    <a-typography-text strong>尺寸:</a-typography-text>
                    <a-input-number v-model="qrSize" :min="128" :max="512" :step="32" style="margin-left: 8px; width: 120px;" />
                  </div>
                </a-space>
              </div>
            </div>
          </a-col>
          <a-col :span="12">
            <div class="output-section">
              <a-typography-text strong>二维码:</a-typography-text>
              <div style="margin-top: 8px; text-align: center; min-height: 200px; display: flex; align-items: center; justify-content: center; border: 1px dashed #d9d9d9; border-radius: 8px;">
                <img v-if="qrOutput" :src="qrOutput" alt="二维码" style="max-width: 100%; max-height: 300px;" />
                <a-typography-text v-else type="secondary">二维码将显示在这里</a-typography-text>
              </div>
            </div>
          </a-col>
        </a-row>
        
        <a-alert 
          v-if="qrError" 
          type="error" 
          :message="qrError"
          style="margin-top: 16px;"
          show-icon
        />
      </a-card>

      <!-- 代码美化工具 -->
      <a-card v-if="activeTab === 'code-format'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            代码美化工具
          </a-typography-title>
        </template>
        <template #extra>
          <a-space>
            <a-select v-model="codeFormatType" style="width: 150px;">
              <a-option value="javascript">JavaScript</a-option>
              <a-option value="css">CSS</a-option>
              <a-option value="html">HTML</a-option>
              <a-option value="xml">XML</a-option>
              <a-option value="sql">SQL</a-option>
            </a-select>
            <a-button type="primary" @click="formatCode">美化</a-button>
          </a-space>
        </template>
        
        <a-row :gutter="24">
          <a-col :span="12">
            <div class="input-section">
              <a-typography-text strong>输入代码:</a-typography-text>
              <a-textarea 
                v-model="codeFormatInput" 
                placeholder="请输入要美化的代码..."
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px; font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;"
              />
            </div>
          </a-col>
          <a-col :span="12">
            <div class="output-section">
              <a-space style="width: 100%; justify-content: space-between;">
                <a-typography-text strong>美化后的代码:</a-typography-text>
                <a-button 
                  v-if="codeFormatOutput" 
                  type="text" 
                  size="small"
                  @click="copyToClipboard(codeFormatOutput)"
                >
                  <template #icon>
                    <icon-copy />
                  </template>
                  复制
                </a-button>
              </a-space>
              <a-textarea 
                v-model="codeFormatOutput" 
                readonly 
                placeholder="美化后的代码将显示在这里..."
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px; font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;"
              />
            </div>
          </a-col>
        </a-row>
        
        <a-alert 
          v-if="codeFormatError" 
          type="error" 
          :message="codeFormatError"
          style="margin-top: 16px;"
          show-icon
        />
      </a-card>

      <!-- 代码压缩工具 -->
      <a-card v-if="activeTab === 'code-compress'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            代码压缩工具
          </a-typography-title>
        </template>
        <template #extra>
          <a-space>
            <a-select v-model="codeCompressType" style="width: 150px;">
              <a-option value="javascript">JavaScript</a-option>
              <a-option value="css">CSS</a-option>
              <a-option value="html">HTML</a-option>
            </a-select>
            <a-button type="primary" @click="compressCode">压缩</a-button>
          </a-space>
        </template>
        
        <a-row :gutter="24">
          <a-col :span="12">
            <div class="input-section">
              <a-typography-text strong>输入代码:</a-typography-text>
              <a-textarea 
                v-model="codeCompressInput" 
                placeholder="请输入要压缩的代码..."
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px; font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;"
              />
            </div>
          </a-col>
          <a-col :span="12">
            <div class="output-section">
              <a-space style="width: 100%; justify-content: space-between;">
                <a-typography-text strong>压缩后的代码:</a-typography-text>
                <a-button 
                  v-if="codeCompressOutput" 
                  type="text" 
                  size="small"
                  @click="copyToClipboard(codeCompressOutput)"
                >
                  <template #icon>
                    <icon-copy />
                  </template>
                  复制
                </a-button>
              </a-space>
              <a-textarea 
                v-model="codeCompressOutput" 
                readonly 
                placeholder="压缩后的代码将显示在这里..."
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px; font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;"
              />
            </div>
          </a-col>
        </a-row>
        
        <a-alert 
          v-if="codeCompressError" 
          type="error" 
          :message="codeCompressError"
          style="margin-top: 16px;"
          show-icon
        />
      </a-card>

      <!-- 字符串编码工具 -->
      <a-card v-if="activeTab === 'string-encode'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            字符串编码工具
          </a-typography-title>
        </template>
        <template #extra>
          <a-space>
            <a-select v-model="stringEncodeType" style="width: 120px;">
              <a-option value="unicode">Unicode</a-option>
              <a-option value="utf8">UTF-8</a-option>
              <a-option value="md5">MD5</a-option>
              <a-option value="sha1">SHA1</a-option>
              <a-option value="sha256">SHA256</a-option>
            </a-select>
            <a-select v-model="stringEncodeMode" style="width: 100px;" v-if="['unicode', 'utf8'].includes(stringEncodeType)">
              <a-option value="encode">编码</a-option>
              <a-option value="decode">解码</a-option>
            </a-select>
            <a-button type="primary" @click="processStringEncode">处理</a-button>
          </a-space>
        </template>
        
        <a-row :gutter="24">
          <a-col :span="12">
            <div class="input-section">
              <a-typography-text strong>输入文本:</a-typography-text>
              <a-textarea 
                v-model="stringEncodeInput" 
                placeholder="请输入要处理的文本..."
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px;"
              />
            </div>
          </a-col>
          <a-col :span="12">
            <div class="output-section">
              <a-space style="width: 100%; justify-content: space-between;">
                <a-typography-text strong>处理结果:</a-typography-text>
                <a-button 
                  v-if="stringEncodeOutput" 
                  type="text" 
                  size="small"
                  @click="copyToClipboard(stringEncodeOutput)"
                >
                  <template #icon>
                    <icon-copy />
                  </template>
                  复制
                </a-button>
              </a-space>
              <a-textarea 
                v-model="stringEncodeOutput" 
                readonly 
                placeholder="处理结果将显示在这里..."
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px; font-family: monospace;"
              />
            </div>
          </a-col>
        </a-row>
        
        <a-alert 
          v-if="stringEncodeError" 
          type="error" 
          :message="stringEncodeError"
          style="margin-top: 16px;"
          show-icon
        />
      </a-card>

      <!-- 密码生成工具 -->
      <a-card v-if="activeTab === 'password-gen'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            随机密码生成器
          </a-typography-title>
        </template>
        <template #extra>
          <a-button type="primary" @click="generatePassword">生成密码</a-button>
        </template>
        
        <a-space direction="vertical" style="width: 100%;" :size="24">
          <a-row :gutter="24">
            <a-col :span="12">
              <a-space direction="vertical" style="width: 100%;" :size="16">
                <div>
                  <a-typography-text strong>密码长度:</a-typography-text>
                  <a-input-number 
                    v-model="passwordLength" 
                    :min="4" 
                    :max="128" 
                    style="margin-left: 12px; width: 120px;" 
                  />
                </div>
                
                <a-space direction="vertical" :size="12">
                  <a-typography-text strong>字符类型:</a-typography-text>
                  <a-checkbox v-model="passwordIncludeUppercase">大写字母 (A-Z)</a-checkbox>
                  <a-checkbox v-model="passwordIncludeLowercase">小写字母 (a-z)</a-checkbox>
                  <a-checkbox v-model="passwordIncludeNumbers">数字 (0-9)</a-checkbox>
                  <a-checkbox v-model="passwordIncludeSymbols">特殊符号 (!@#$%^&*)</a-checkbox>
                </a-space>
              </a-space>
            </a-col>
            <a-col :span="12">
              <div style="text-align: center; padding: 40px 0;">
                <a-typography-text strong style="display: block; margin-bottom: 16px;">
                  生成的密码:
                </a-typography-text>
                
                <a-input-group style="max-width: 400px; margin: 0 auto;">
                  <a-input 
                    v-model="generatedPassword" 
                    readonly 
                    placeholder="点击生成密码..."
                    style="text-align: center; font-family: monospace; font-size: 16px;"
                  />
                  <a-button 
                    v-if="generatedPassword" 
                    type="primary"
                    @click="copyToClipboard(generatedPassword)"
                  >
                    <template #icon>
                      <icon-copy />
                    </template>
                    复制
                  </a-button>
                </a-input-group>
              </div>
            </a-col>
          </a-row>
        </a-space>
      </a-card>

      <!-- 颜色工具 -->
      <a-card v-if="activeTab === 'color-tools'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            颜色转换工具
          </a-typography-title>
        </template>
        <template #extra>
          <a-space>
            <a-select v-model="colorFormat" style="width: 120px;">
              <a-option value="hex">HEX</a-option>
              <a-option value="rgb">RGB</a-option>
              <a-option value="hsl">HSL</a-option>
              <a-option value="hsv">HSV</a-option>
            </a-select>
            <a-button type="primary" @click="convertColor">转换</a-button>
          </a-space>
        </template>
        
        <a-row :gutter="24">
          <a-col :span="12">
            <div class="input-section">
              <a-typography-text strong>输入颜色:</a-typography-text>
              <a-space style="margin-top: 8px; width: 100%;" direction="vertical" :size="12">
                <a-input 
                  v-model="colorInput" 
                  placeholder="请输入十六进制颜色值 (#RRGGBB)"
                />
                <div style="display: flex; align-items: center; gap: 12px;">
                  <a-typography-text>颜色预览:</a-typography-text>
                  <div 
                    :style="{ 
                      width: '60px', 
                      height: '40px', 
                      backgroundColor: colorInput, 
                      border: '1px solid #d9d9d9', 
                      borderRadius: '4px' 
                    }"
                  ></div>
                  <input 
                    type="color" 
                    v-model="colorInput" 
                    style="width: 40px; height: 40px; border: none; border-radius: 4px; cursor: pointer;"
                  >
                </div>
              </a-space>
            </div>
          </a-col>
          <a-col :span="12">
            <div class="output-section">
              <a-space style="width: 100%; justify-content: space-between;">
                <a-typography-text strong>转换结果:</a-typography-text>
                <a-button 
                  v-if="colorOutput" 
                  type="text" 
                  size="small"
                  @click="copyToClipboard(colorOutput)"
                >
                  <template #icon>
                    <icon-copy />
                  </template>
                  复制
                </a-button>
              </a-space>
              <a-input 
                v-model="colorOutput" 
                readonly 
                placeholder="转换结果将显示在这里..."
                style="margin-top: 8px; font-family: monospace; font-size: 16px;"
              />
            </div>
          </a-col>
        </a-row>
        
        <a-alert 
          v-if="colorError" 
          type="error" 
          :message="colorError"
          style="margin-top: 16px;"
          show-icon
        />
      </a-card>

      <!-- Markdown工具 -->
      <a-card v-if="activeTab === 'markdown'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            Markdown工具
          </a-typography-title>
        </template>
        <template #extra>
          <a-space>
            <a-select v-model="markdownMode" style="width: 120px;">
              <a-option value="preview">预览</a-option>
              <a-option value="html">转HTML</a-option>
            </a-select>
            <a-button type="primary" @click="processMarkdown">处理</a-button>
          </a-space>
        </template>
        
        <a-row :gutter="24">
          <a-col :span="12">
            <div class="input-section">
              <a-typography-text strong>Markdown输入:</a-typography-text>
              <a-textarea 
                v-model="markdownInput" 
                placeholder="请输入Markdown文本..."
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px; font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;"
              />
            </div>
          </a-col>
          <a-col :span="12">
            <div class="output-section">
              <a-space style="width: 100%; justify-content: space-between;">
                <a-typography-text strong>{{ markdownMode === 'preview' ? '预览' : 'HTML代码' }}:</a-typography-text>
                <a-button 
                  v-if="markdownOutput && markdownMode === 'html'" 
                  type="text" 
                  size="small"
                  @click="copyToClipboard(markdownOutput)"
                >
                  <template #icon>
                    <icon-copy />
                  </template>
                  复制
                </a-button>
              </a-space>
              <div 
                v-if="markdownMode === 'preview'" 
                v-html="markdownOutput"
                style="margin-top: 8px; padding: 16px; border: 1px solid #d9d9d9; border-radius: 8px; background: #fafafa; min-height: 400px; overflow-y: auto;"
              ></div>
              <a-textarea 
                v-else
                v-model="markdownOutput" 
                readonly 
                placeholder="HTML代码将显示在这里..."
                :rows="15"
                :auto-size="{ minRows: 15, maxRows: 20 }"
                style="margin-top: 8px; font-family: 'Monaco', 'Menlo', 'Ubuntu Mono', monospace;"
              />
            </div>
          </a-col>
        </a-row>
      </a-card>

      <!-- API测试工具 -->
      <a-card v-if="activeTab === 'api-test'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            API测试工具
          </a-typography-title>
        </template>
        <template #extra>
          <a-button type="primary" @click="testAPI" :loading="apiLoading">发送请求</a-button>
        </template>
        
        <a-space direction="vertical" style="width: 100%;" :size="16">
          <a-row :gutter="16">
            <a-col :span="4">
              <a-select v-model="apiMethod" style="width: 100%;">
                <a-option value="GET">GET</a-option>
                <a-option value="POST">POST</a-option>
                <a-option value="PUT">PUT</a-option>
                <a-option value="DELETE">DELETE</a-option>
                <a-option value="PATCH">PATCH</a-option>
                <a-option value="HEAD">HEAD</a-option>
              </a-select>
            </a-col>
            <a-col :span="20">
              <a-input 
                v-model="apiUrl" 
                placeholder="请输入API地址 (https://api.example.com/users)"
              />
            </a-col>
          </a-row>
          
          <div>
            <a-typography-text strong>请求头 (JSON格式):</a-typography-text>
            <a-textarea 
              v-model="apiHeaders" 
              placeholder='{"Authorization": "Bearer token", "Content-Type": "application/json"}'
              :rows="4"
              style="margin-top: 8px; font-family: monospace;"
            />
          </div>
          
          <div v-if="apiMethod !== 'GET'">
            <a-typography-text strong>请求体:</a-typography-text>
            <a-textarea 
              v-model="apiBody" 
              placeholder="请输入请求体内容..."
              :rows="6"
              style="margin-top: 8px; font-family: monospace;"
            />
          </div>
          
          <div>
            <a-space style="width: 100%; justify-content: space-between;">
              <a-typography-text strong>响应结果:</a-typography-text>
              <a-button 
                v-if="apiResponse" 
                type="text" 
                size="small"
                @click="copyToClipboard(apiResponse)"
              >
                <template #icon>
                  <icon-copy />
                </template>
                复制
              </a-button>
            </a-space>
            <a-textarea 
              v-model="apiResponse" 
              readonly 
              placeholder="响应结果将显示在这里..."
              :rows="12"
              :auto-size="{ minRows: 12, maxRows: 20 }"
              style="margin-top: 8px; font-family: monospace;"
            />
          </div>
        </a-space>
        
        <a-alert 
          v-if="apiError" 
          type="error" 
          :message="apiError"
          style="margin-top: 16px;"
          show-icon
        />
      </a-card>

      <!-- 便签笔记工具 -->
      <a-card v-if="activeTab === 'notes'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            便签笔记
          </a-typography-title>
        </template>
        <template #extra>
          <a-button type="primary" @click="saveNote">保存笔记</a-button>
        </template>
        
        <a-row :gutter="24">
          <a-col :span="8">
            <div>
              <a-typography-text strong>笔记列表:</a-typography-text>
              <div style="margin-top: 8px; max-height: 500px; overflow-y: auto;">
                <a-card 
                  v-for="note in notesList" 
                  :key="note.id"
                  size="small"
                  style="margin-bottom: 8px; cursor: pointer;"
                  :class="{ 'selected-note': selectedNoteId === note.id }"
                  @click="loadNote(note.id)"
                >
                  <template #extra>
                    <a-button 
                      type="text" 
                      size="small" 
                      status="danger"
                      @click.stop="deleteNote(note.id)"
                    >
                      删除
                    </a-button>
                  </template>
                  <a-typography-text strong>{{ note.title }}</a-typography-text>
                  <br>
                  <a-typography-text type="secondary" style="font-size: 12px;">
                    {{ note.createdAt }}
                  </a-typography-text>
                </a-card>
                
                <a-empty v-if="notesList.length === 0" description="暂无笔记" />
              </div>
            </div>
          </a-col>
          <a-col :span="16">
            <a-space direction="vertical" style="width: 100%;" :size="16">
              <div>
                <a-typography-text strong>笔记标题:</a-typography-text>
                <a-input 
                  v-model="noteTitle" 
                  placeholder="请输入笔记标题..."
                  style="margin-top: 8px;"
                />
              </div>
              
              <div>
                <a-typography-text strong>笔记内容:</a-typography-text>
                <a-textarea 
                  v-model="noteContent" 
                  placeholder="请输入笔记内容..."
                  :rows="15"
                  :auto-size="{ minRows: 15, maxRows: 20 }"
                  style="margin-top: 8px;"
                />
              </div>
            </a-space>
          </a-col>
        </a-row>
      </a-card>

      <!-- 图片工具 -->
      <a-card v-if="activeTab === 'image-tools'" class="tool-panel" :bordered="false">
        <template #title>
          <a-typography-title :heading="3" style="margin: 0;">
            图片工具
          </a-typography-title>
        </template>
        
        <a-space direction="vertical" style="width: 100%;" :size="24">
          <div>
            <a-typography-text strong>选择图片:</a-typography-text>
            <input 
              type="file" 
              accept="image/*" 
              @change="handleImageUpload"
              style="margin-top: 8px; width: 100%;"
            />
          </div>
          
          <div v-if="imagePreview">
            <a-typography-text strong>图片预览:</a-typography-text>
            <div style="margin-top: 8px; text-align: center;">
              <img 
                :src="imagePreview" 
                alt="图片预览" 
                style="max-width: 100%; max-height: 400px; border: 1px solid #d9d9d9; border-radius: 8px;"
              />
            </div>
          </div>
          
          <a-row :gutter="24" v-if="imagePreview">
            <a-col :span="12">
              <a-space direction="vertical" style="width: 100%;" :size="12">
                <div>
                  <a-typography-text strong>输出格式:</a-typography-text>
                  <a-select v-model="imageFormat" style="margin-left: 12px; width: 120px;">
                    <a-option value="png">PNG</a-option>
                    <a-option value="jpg">JPG</a-option>
                    <a-option value="webp">WebP</a-option>
                  </a-select>
                </div>
                
                <div v-if="imageFormat !== 'png'">
                  <a-typography-text strong>图片质量:</a-typography-text>
                  <a-slider 
                    v-model="imageQuality" 
                    :min="0.1" 
                    :max="1" 
                    :step="0.1" 
                    style="margin-left: 12px; width: 200px;"
                  />
                  <span style="margin-left: 8px;">{{ Math.round(imageQuality * 100) }}%</span>
                </div>
              </a-space>
            </a-col>
            <a-col :span="12">
              <a-space>
                <a-button type="primary">转换格式</a-button>
                <a-button>压缩图片</a-button>
                <a-button>调整尺寸</a-button>
              </a-space>
            </a-col>
          </a-row>
        </a-space>
        
        <a-alert 
          v-if="imageError" 
          type="error" 
          :message="imageError"
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

/* 新工具样式 */
.selected-note {
  border: 2px solid #1890ff !important;
  background-color: #f0f8ff !important;
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