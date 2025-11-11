<!-- markdownlint-disable MD033 MD041 MD045 MD026 -->
<p align="center" dir="auto">
    <img width="128" height="128" src="https://s2.loli.net/2025/03/10/GSsjOcHqdtBkyu9.png" alt="Logo"/>
</p>

<h1 align="center" tabindex="-1" class="heading-element" dir="auto">OsynicOsuapi</h1>

<p align="center">
  <a href="https://www.npmjs.com/package/@osynicite/osynic-osuapi" target="_blank"><img src="https://img.shields.io/npm/v/@osynicite/osynic-osuapi"/></a>
  <a href="https://www.npmjs.com/package/@osynicite/osynic-osuapi" target="_blank"><img src="https://img.shields.io/npm/dm/@osynicite/osynic-osuapi"/></a>
  <a href="https://hakochest.github.io/osynic-osuapi/" target="_blank"><img src="https://img.shields.io/badge/Typedoc-Documentation-blue"/></a>
  <a href="https://github.com/osynicite/osynic_osuapi" target="_blank"><img src="https://img.shields.io/badge/License-MIT-green.svg"/></a>
  <a href="https://discord.gg/DRnZSES3BC" target="_blank"><img src="https://img.shields.io/badge/chat-discord-7289da.svg"/></a>
</p>

<p align="center">
    🚀 High Performance WebAssembly · 📦 Out of the Box · 🎯 Native Browser Support<br/>
    Complete osu! API client library WASM bindings, empowering your web applications
</p>

<p align="center">
  <a href="./README.md">🇨🇳 中文</a> ·
  <a href="./README_EN.md">🇺🇸 English</a>
</p>

<hr />

# 📚 Table of Contents

- [✨ Features](#-features)
- [🚀 Quick Start](#-quick-start)
- [📖 Usage Guide](#-usage-guide)
- [🎯 API Support](#-api-support)
- [⚡ Performance Features](#-performance-features)
- [🔧 Integration Examples](#-integration-examples)
- [⚠️ Special Attention](#️-special-attention)
- [🤝 Contribution Guidelines](#-contribution-guidelines)
- [📜 License](#-license)

# 🧻 [API Experience Website](https://osynic-osuapi.deno.dev/)

[![OSUAPIV1EN.png](https://s2.loli.net/2025/11/05/qTHoNL69icrlJgA.png)](https://osynic-osuapi.deno.dev/)

# ✨ Features

- **🌐 WASM Native Support**: Compiled to WebAssembly, runs at near-native speed in browsers
- **📱 Multi-Framework Support**: Compatible with Vue, React, Svelte and other popular frontend frameworks
- **🔄 Complete API Support**: All major V1 and V2 API endpoints (except undocumented interfaces)
- **🎨 TypeScript Friendly**: Complete type definitions for the best development experience
- **⚡ Zero Configuration**: Install via npm and use immediately, simple and fast integration
- **🌍 CORS Handling**: Built-in CORS proxy support to handle browser cross-origin issues
- **📊 Lightweight**: Optimized WASM package size for quick loading and initialization

# 🚀 Quick Start

## Step 1: Install Dependencies

Install using npm, yarn, or pnpm:

```bash
# npm
npm install @osynicite/osynic-osuapi

# yarn
yarn add @osynicite/osynic-osuapi

# pnpm
pnpm add @osynicite/osynic-osuapi
```

## Step 2: Basic Configuration

Ensure your project is configured to support WebAssembly:

### Vite Configuration

```typescript
// vite.config.ts
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'

export default defineConfig({
  plugins: [
    wasm(),
    topLevelAwait(),
    vue(),
  ],
})
```

## Step 3: Obtain API Authorization

Visit your [osu! settings page](https://osu.ppy.sh/home/account/edit) and apply for the appropriate API authorization:

- [**V2 API**](https://osu.ppy.sh/docs/index.html): Apply in the "OAuth" section
- [**V1 API**](https://github.com/ppy/osu-api/wiki): Apply in the "Legacy API" section

# 📖 Usage Guide

## V2 API - Get User Information

### Using Client Credentials Grant Authentication

```typescript
import { OsynicOsuApiV2GlooClient } from '@osynicite/osynic-osuapi'

async function getUserInfo() {
  const client = new OsynicOsuApiV2GlooClient()
  client.set_proxy_url('YOUR_CORS_PROXY_URL') // You can see https://github.com/Islatri/deno_osynic_cors

  const token = await client.oauth.getTokenWithoutCode(
    'YOUR_CLIENT_ID_HERE',
    'YOUR_CLIENT_SECRET_HERE'
  )
  
  // Get user information
  const user = await client.users.getUserByUsername('peppy')
  console.log(user)
}

getUserInfo().catch(console.error)
```

### Using Authorization Code Grant Authentication

This part should be implemented in the backend to protect `client_secret` from being exposed in the frontend. For specific backend implementation, refer to the [osynic-oauth](https://github.com/Islatri/deno_osynic_oauth) project, with deployment example at [OsynicOauth](https://osynic-oauth.deno.dev).

The frontend part can refer to the implementation in examples.

## V1 API - Query Beatmap Information

```typescript
import { OsynicOsuApiV1GlooClient } from '@osynicite/osynic-osuapi'

async function getBeatmapInfo() {
  const client = new OsynicOsuApiV1GlooClient('YOUR_API_KEY_HERE')
  client.set_proxy_url('YOUR_CORS_PROXY_URL') // You can see https://github.com/Islatri/deno_osynic_cors

  // Query beatmap by hash
  const beatmaps = await client.beatmap.getBeatmaps({
    hash: '69f77b0dfe67d288c1bf748f91ceb133'
  })
  
  console.log(beatmaps)
}

getBeatmapInfo().catch(console.error)
```

## Vue 3 Integration Example

```vue
<template>
    <div class="p-6 bg-gray-900 text-white min-h-screen">
        <div class="max-w-2xl mx-auto space-y-4">
            <div class="space-y-2">
                <input v-model="query.bid" type="text" placeholder="Beatmap ID"
                    class="w-full px-3 py-2 bg-gray-800 rounded border border-gray-700" />
                <input v-model="query.sid" type="text" placeholder="Beatmapset ID"
                    class="w-full px-3 py-2 bg-gray-800 rounded border border-gray-700" />
                <select v-model="query.mode" class="w-full px-3 py-2 bg-gray-800 rounded border border-gray-700">
                    <option value="">All Modes</option>
                    <option value="0">Standard</option>
                    <option value="1">Taiko</option>
                    <option value="2">Catch</option>
                    <option value="3">Mania</option>
                </select>
                <button @click="search" :disabled="loading"
                    class="w-full px-3 py-2 bg-blue-600 hover:bg-blue-700 rounded disabled:opacity-50">
                    {{ loading ? 'Loading...' : 'Search' }}
                </button>
            </div>
            <div v-if="error" class="text-red-400">{{ error }}</div>
            <div v-if="beatmaps.length" class="space-y-2">
                <div v-for="m in beatmaps" :key="m.beatmap_id" class="bg-gray-800 p-3 rounded text-sm">
                    <div class="font-bold text-blue-300">{{ m.title }} [{{ m.version }}]</div>
                    <div class="text-gray-400">★{{ parseFloat(m.difficultyrating).toFixed(2) }} | {{ m.artist }}</div>
                    <div class="text-gray-500 text-xs mt-1">
                        {{ formatTime(m.total_length) }} | BPM {{ parseInt(m.bpm) }} | {{ calcPassRate(m.playcount,
                        m.passcount) }}% Pass Rate
                    </div>
                </div>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';
import { OsynicOsuApiV1GlooClient } from '@osynicite/osynic-osuapi';

const client = new OsynicOsuApiV1GlooClient("YOUR_API_KEY_HERE_AND_PLS_NOT_SHARE_IT_IN_YOUR_CONCRETE_PROJECT");
client.setProxyUrl("YOUR_PROXY_URL_HERE_BECAUSE_CORS"); // You can see https://github.com/Islatri/deno_osynic_cors

const query = reactive({ bid: '', sid: '', mode: '' });
const beatmaps = ref([]);
const loading = ref(false);
const error = ref('');

const search = async () => {
    loading.value = true;
    error.value = '';
    try {
        const params = Object.fromEntries(Object.entries(query).filter(([, v]) => v));
        beatmaps.value = await client.getBeatmaps(params).then(r => Array.isArray(r) ? r : [r]);
    } catch (err: any) {
        error.value = err?.message || 'Query failed';
    } finally {
        loading.value = false;
    }
};

const formatTime = (s: string) => {
    const t = parseInt(s);
    return `${Math.floor(t / 60)}:${(t % 60).toString().padStart(2, '0')}`;
};

const calcPassRate = (p: string, pa: string) => ((parseInt(pa) / parseInt(p)) * 100).toFixed(1);
</script>
```

# 🎯 API Support

## V1 API Support

| API              | Support | Description       |
| ---------------- | ------- | ------------------ |
| /get_beatmaps    | ✅       | Get beatmaps       |
| /get_user        | ✅       | Get user           |
| /get_user_best   | ✅       | Get user's best    |
| /get_user_recent | ✅       | Get user's recent  |
| /get_match       | ✅       | Get match          |
| /get_scores      | ✅       | Get scores         |
| /get_replay      | ✅       | Get replay         |

## V2 API Support (Main Endpoints)

| Category       | Support Status          | Description             |
| -------------- | ----------------------- | ----------------------- |
| Authentication | ✅ 4/4                   | OAuth Authentication    |
| Beatmaps       | ✅ 10/10                 | Beatmap API             |
| Beatmapsets    | ⚠️ 2/7 (partial auth)   | Beatmapset API          |
| Changelog      | ✅ 3/3                   | Changelog               |
| Comments       | ⚠️ 2/7 (partial auth)   | Comments API            |
| Events         | ✅ 1/1                   | Events API              |
| Forum          | ⚠️ 4/8 (partial auth)   | Forum API               |
| Matches        | ✅ 2/2                   | Matches API             |
| News           | ✅ 2/2                   | News API                |
| Rankings       | ✅ 3/3                   | Rankings API            |
| Scores         | ✅ 1/1                   | Scores API              |
| Users          | ✅ 7/7                   | Users API               |
| Wiki           | ✅ 1/1                   | Wiki API                |
| Friends        | ✅ 2/2                   | Friends API             |

> For more details, see the main project's [API Checklist](https://github.com/osynicite/osynic_osuapi#-api-checklist)

# ⚡ Performance Features

## Why Use WASM Version?

- **🚀 High Performance**: Significant performance advantages over JavaScript, especially for complex computations
- **💾 Small Package Size**: Optimized WASM package is smaller than full JavaScript libraries
- **🔄 No Dependencies**: No dependency on other JavaScript libraries, can be used directly in any project
- **🌐 Native Browser Support**: Modern browsers support WebAssembly natively, no additional configuration needed

# 🔧 Integration Examples

## Project Examples

This library provides complete project examples located in the `wasm/examples/` directory:

- **Vue 3 Project**: Complete example using Vue 3 + TypeScript + Vite
- Feature demonstrations: user queries, beatmap search, score viewing, etc.

### Running Example Projects

```bash
cd wasm/examples
npm install
npm run dev
```

## Using in Existing Projects

### Existing Vite Project

```bash
npm install @osynicite/osynic-osuapi
```

Then in your project:

```typescript
import { OsynicOsuApiV2Client } from '@osynicite/osynic-osuapi'

const client = new OsynicOsuApiV2Client()
client.set_proxy_url('YOUR_CORS_PROXY_URL') // You can see https://github.com/Islatri/deno_osynic_cors
```

# ⚠️ Special Attention

## Browser CORS Limitations

Due to browser cross-origin policy restrictions, JavaScript clients cannot directly access the osu! API. This library exposes the `set_proxy_url` method, allowing you to set a CORS proxy URL to bypass this limitation. For information on building and using CORS proxy servers, refer to [deno_osynic_cors](https://github.com/Islatri/deno_osynic_cors).

## Data Structure Changes

The most common issues when using this library stem from changes in the official osu! API entity structure:

- **🔄 Entity Structure Changes**: The structure of osu! API may change at any time, official documentation updates may not be timely
- **📝 Return Field Changes**: Return fields of some interfaces may change, especially for less frequently used endpoints
- **❓ Unexpected Null Values**: Some fields may return null under certain conditions, but are not marked as optional in documentation

## Issue Reporting

If you encounter issues while using this library, please submit an Issue with:

1. **API endpoint used**
2. **Request parameters**
3. **Error message or network response**
4. **Browser console output**

## FAQ

### Q: How to safely use the API in production?

A: Strongly recommend using backend proxy or V2 Authorization Code Grant approach in production, rather than directly exposing `client_secret` in the browser.

### Q: Does it support CommonJS?

A: This library mainly supports ES modules. If you need to use CommonJS, ensure your build tool supports ESM to CJS conversion.

### Q: Can it be used in Node.js?

A: Not recommended to use WASM version in Node.js as WASM support for Node.js is not yet mature. However, you can use it in various frontend frameworks through plugins like `vite-plugin-wasm` and `vite-plugin-top-level-await`.

# 🤝 Contribution Guidelines

We welcome contributions! If you find any issues or have suggestions for improvement, please follow these guidelines:

## How to Contribute

1. **Fork the project** - Fork the project on GitHub
2. **Create a branch** - `git checkout -b feature/your-feature`
3. **Commit changes** - `git commit -am 'Add your feature'`
4. **Push to branch** - `git push origin feature/your-feature`
5. **Submit Pull Request** - Create a new Pull Request

## Development Guide

### Compile WASM Library

```bash
# From project root directory
cd wasm

# Build WASM library
wasm-pack build --release --target bundler --out-dir pkg --scope osynicite
```

### Build NPM Package

```bash
# Install dependencies
npm install

# Publish (maintainers)
npm publish
```

## Code Standards

- Follow official Rust coding conventions
- New features must include test cases
- Run `cargo fmt` and `cargo clippy` before submitting
- Update relevant documentation and examples

# 📜 License

This project is open source under the [MIT License](./LICENSE).

---

## Related Resources

- 📚 [Rust Documentation](https://github.com/osynicite/osynic_osuapi) - Complete Rust library
- 🌐 [Online Experience](https://osynic-osuapi.deno.dev) - Online API experience platform
- 📖 [V1 API Documentation](https://github.com/ppy/osu-api/wiki) - osu! API V1 Official Documentation
- 📖 [V2 API Documentation](https://osu.ppy.sh/docs/index.html) - osu! API V2 Official Documentation
- 💬 [Discord Community](https://discord.gg/DRnZSES3BC) - Join our community
