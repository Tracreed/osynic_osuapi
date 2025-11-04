<template>
  <div class="min-h-screen bg-linear-to-br from-purple-900 via-blue-900 to-indigo-900 text-white">
    <div class="container mx-auto px-4 py-8 max-w-6xl">
      <!-- Header -->
      <div class="text-center mb-12">
        <div class="flex items-center justify-center gap-3 mb-4">
          <Music class="w-12 h-12 text-pink-400" />
          <h1 class="text-5xl font-bold bg-linear-to-r from-pink-400 to-purple-400 bg-clip-text text-transparent">
            Osynic OsuApi Client
          </h1>
        </div>
        <p class="text-gray-300 text-lg">Search and explore osu! beatmaps</p>
      </div>

      <!-- Search Form -->
      <div class="bg-white/10 backdrop-blur-lg rounded-2xl p-8 shadow-2xl mb-8">
        <h2 class="text-2xl font-semibold mb-6 flex items-center gap-2">
          <Search class="w-6 h-6 text-pink-400" />
          Search Beatmaps
        </h2>
        
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-6">
          <!-- Beatmap ID (bid) -->
          <div>
            <label class="block text-sm font-medium mb-2 text-gray-200">Beatmap ID</label>
            <div class="relative">
              <Hash class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
              <input
                v-model="searchParams.bid"
                type="text"
                placeholder="e.g., 75"
                class="w-full pl-10 pr-4 py-3 bg-white/5 border border-white/20 rounded-lg focus:ring-2 focus:ring-pink-400 focus:border-transparent outline-none transition"
              />
            </div>
          </div>

          <!-- Beatmapset ID (sid) -->
          <div>
            <label class="block text-sm font-medium mb-2 text-gray-200">Beatmapset ID</label>
            <div class="relative">
              <Layers class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
              <input
                v-model="searchParams.sid"
                type="text"
                placeholder="e.g., 1"
                class="w-full pl-10 pr-4 py-3 bg-white/5 border border-white/20 rounded-lg focus:ring-2 focus:ring-pink-400 focus:border-transparent outline-none transition"
              />
            </div>
          </div>

          <!-- User ID (uid) -->
          <div>
            <label class="block text-sm font-medium mb-2 text-gray-200">User ID</label>
            <div class="relative">
              <User class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
              <input
                v-model="searchParams.uid"
                type="text"
                placeholder="User ID"
                class="w-full pl-10 pr-4 py-3 bg-white/5 border border-white/20 rounded-lg focus:ring-2 focus:ring-pink-400 focus:border-transparent outline-none transition"
              />
            </div>
          </div>

          <!-- Mode -->
          <div>
            <label class="block text-sm font-medium mb-2 text-gray-200">Game Mode</label>
            <div class="relative">
              <Gamepad2 class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
              <select
                v-model.number="searchParams.mode"
                class="w-full pl-10 pr-4 py-3 bg-white/5 border border-white/20 rounded-lg focus:ring-2 focus:ring-pink-400 focus:border-transparent outline-none transition appearance-none cursor-pointer"
              >
                <option :value="null">All Modes</option>
                <option :value="0">osu! Standard</option>
                <option :value="1">Taiko</option>
                <option :value="2">Catch the Beat</option>
                <option :value="3">osu!mania</option>
              </select>
            </div>
          </div>

          <!-- Type -->
          <div>
            <label class="block text-sm font-medium mb-2 text-gray-200">Type</label>
            <div class="relative">
              <Filter class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
              <select
                v-model="searchParams.typee"
                class="w-full pl-10 pr-4 py-3 bg-white/5 border border-white/20 rounded-lg focus:ring-2 focus:ring-pink-400 focus:border-transparent outline-none transition appearance-none cursor-pointer"
              >
                <option :value="null">All Types</option>
                <option value="id">By ID</option>
                <option value="name">By Name</option>
              </select>
            </div>
          </div>

          <!-- Limit -->
          <div>
            <label class="block text-sm font-medium mb-2 text-gray-200">Limit</label>
            <div class="relative">
              <ListFilter class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
              <input
                v-model.number="searchParams.limit"
                type="number"
                min="1"
                max="500"
                placeholder="Max results (default: 500)"
                class="w-full pl-10 pr-4 py-3 bg-white/5 border border-white/20 rounded-lg focus:ring-2 focus:ring-pink-400 focus:border-transparent outline-none transition"
              />
            </div>
          </div>

          <!-- Mods -->
          <div>
            <label class="block text-sm font-medium mb-2 text-gray-200">Mods (bitwise)</label>
            <div class="relative">
              <Sparkles class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
              <input
                v-model.number="searchParams.mods"
                type="number"
                placeholder="e.g., 24 for HDDT"
                class="w-full pl-10 pr-4 py-3 bg-white/5 border border-white/20 rounded-lg focus:ring-2 focus:ring-pink-400 focus:border-transparent outline-none transition"
              />
            </div>
          </div>

          <!-- Hash -->
          <div>
            <label class="block text-sm font-medium mb-2 text-gray-200">Hash (MD5)</label>
            <div class="relative">
              <Key class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
              <input
                v-model="searchParams.hash"
                type="text"
                placeholder="File MD5 hash"
                class="w-full pl-10 pr-4 py-3 bg-white/5 border border-white/20 rounded-lg focus:ring-2 focus:ring-pink-400 focus:border-transparent outline-none transition"
              />
            </div>
          </div>

          <!-- Has Converted -->
          <div>
            <label class="block text-sm font-medium mb-2 text-gray-200">Has Converted</label>
            <div class="relative">
              <ToggleLeft class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
              <select
                v-model.number="searchParams.has_converted"
                class="w-full pl-10 pr-4 py-3 bg-white/5 border border-white/20 rounded-lg focus:ring-2 focus:ring-pink-400 focus:border-transparent outline-none transition appearance-none cursor-pointer"
              >
                <option :value="null">Any</option>
                <option :value="0">No</option>
                <option :value="1">Yes</option>
              </select>
            </div>
          </div>

          <!-- Since Date -->
          <div>
            <label class="block text-sm font-medium mb-2 text-gray-200">Since Date</label>
            <div class="relative">
              <Calendar class="absolute left-3 top-1/2 -translate-y-1/2 w-5 h-5 text-gray-400" />
              <input
                v-model="searchParams.since"
                type="date"
                class="w-full pl-10 pr-4 py-3 bg-white/5 border border-white/20 rounded-lg focus:ring-2 focus:ring-pink-400 focus:border-transparent outline-none transition"
              />
            </div>
          </div>
        </div>

        <!-- Action Buttons -->
        <div class="flex gap-4">
          <button
            @click="searchBeatmaps"
            :disabled="loading"
            class="flex-1 flex items-center justify-center gap-2 bg-linear-to-r from-pink-500 to-purple-500 hover:from-pink-600 hover:to-purple-600 disabled:from-gray-500 disabled:to-gray-600 disabled:cursor-not-allowed py-3 px-6 rounded-lg font-semibold transition-all transform hover:scale-105 active:scale-95"
          >
            <Search class="w-5 h-5" />
            {{ loading ? 'Searching...' : 'Search Beatmaps' }}
          </button>
          
          <button
            @click="clearSearch"
            class="flex items-center justify-center gap-2 bg-white/10 hover:bg-white/20 py-3 px-6 rounded-lg font-semibold transition-all"
          >
            <X class="w-5 h-5" />
            Clear
          </button>
        </div>
      </div>

      <!-- Error Message -->
      <div v-if="error" class="bg-red-500/20 border border-red-500/50 rounded-xl p-4 mb-8 flex items-start gap-3">
        <AlertCircle class="w-6 h-6 text-red-400 shrink-0 mt-0.5" />
        <div>
          <h3 class="font-semibold text-red-300 mb-1">Error</h3>
          <p class="text-red-200">{{ error }}</p>
        </div>
      </div>

      <!-- Loading State -->
      <div v-if="loading" class="flex items-center justify-center py-20">
        <Loader2 class="w-12 h-12 text-pink-400 animate-spin" />
      </div>

      <!-- Results -->
      <div v-else-if="beatmaps && beatmaps.length > 0" class="space-y-4">
        <div class="flex items-center justify-between mb-6">
          <h2 class="text-2xl font-semibold flex items-center gap-2">
            <FileMusic class="w-6 h-6 text-pink-400" />
            Results ({{ beatmaps.length }})
          </h2>
        </div>

        <div class="grid grid-cols-1 gap-4">
          <div
            v-for="beatmap in beatmaps"
            :key="beatmap.beatmap_id"
            class="bg-white/10 backdrop-blur-lg rounded-xl p-6 hover:bg-white/15 transition-all border border-white/10 hover:border-pink-400/50"
          >
            <!-- Header -->
            <div class="flex items-start justify-between mb-4">
              <div class="flex-1">
                <h3 class="text-xl font-bold mb-1 text-pink-300">{{ beatmap.title }}</h3>
                <p class="text-gray-300 mb-2">{{ beatmap.artist }}</p>
                <div class="flex items-center gap-2 flex-wrap">
                  <span class="text-sm text-gray-400">by {{ beatmap.creator }}</span>
                  <span class="text-gray-600">•</span>
                  <span class="text-sm font-semibold text-purple-300">{{ beatmap.version }}</span>
                </div>
              </div>
              <div class="flex flex-col items-end gap-2">
                <span :class="getDifficultyColor(beatmap.difficultyrating)" class="px-3 py-1 rounded-full text-xs font-semibold">
                  ★ {{ parseFloat(beatmap.difficultyrating).toFixed(2) }}
                </span>
                <span class="px-3 py-1 bg-blue-500/30 text-blue-200 rounded-full text-xs font-semibold">
                  {{ getModeName(beatmap.mode) }}
                </span>
                <span :class="getApprovedColor(beatmap.approved)" class="px-3 py-1 rounded-full text-xs font-semibold">
                  {{ getApprovedStatus(beatmap.approved) }}
                </span>
              </div>
            </div>

            <!-- Stats Grid -->
            <div class="grid grid-cols-2 md:grid-cols-4 gap-3 mb-4">
              <div class="bg-white/5 rounded-lg p-3">
                <div class="flex items-center gap-2 mb-1">
                  <Clock class="w-4 h-4 text-gray-400" />
                  <span class="text-xs text-gray-400">Length</span>
                </div>
                <p class="font-semibold">{{ formatDuration(beatmap.total_length) }}</p>
              </div>

              <div class="bg-white/5 rounded-lg p-3">
                <div class="flex items-center gap-2 mb-1">
                  <Music class="w-4 h-4 text-gray-400" />
                  <span class="text-xs text-gray-400">BPM</span>
                </div>
                <p class="font-semibold">{{ parseFloat(beatmap.bpm).toFixed(0) }}</p>
              </div>

              <div class="bg-white/5 rounded-lg p-3">
                <div class="flex items-center gap-2 mb-1">
                  <Target class="w-4 h-4 text-gray-400" />
                  <span class="text-xs text-gray-400">Max Combo</span>
                </div>
                <p class="font-semibold">{{ beatmap.max_combo }}x</p>
              </div>

              <div class="bg-white/5 rounded-lg p-3">
                <div class="flex items-center gap-2 mb-1">
                  <Heart class="w-4 h-4 text-gray-400" />
                  <span class="text-xs text-gray-400">Favorites</span>
                </div>
                <p class="font-semibold">{{ beatmap.favourite_count }}</p>
              </div>
            </div>

            <!-- Difficulty Stats -->
            <div class="grid grid-cols-2 md:grid-cols-4 gap-3 mb-4">
              <div class="bg-purple-500/10 rounded-lg p-2 border border-purple-500/20">
                <span class="text-xs text-purple-300">CS</span>
                <p class="font-bold text-purple-200">{{ parseFloat(beatmap.diff_size).toFixed(1) }}</p>
              </div>
              <div class="bg-blue-500/10 rounded-lg p-2 border border-blue-500/20">
                <span class="text-xs text-blue-300">AR</span>
                <p class="font-bold text-blue-200">{{ parseFloat(beatmap.diff_approach).toFixed(1) }}</p>
              </div>
              <div class="bg-green-500/10 rounded-lg p-2 border border-green-500/20">
                <span class="text-xs text-green-300">OD</span>
                <p class="font-bold text-green-200">{{ parseFloat(beatmap.diff_overall).toFixed(1) }}</p>
              </div>
              <div class="bg-red-500/10 rounded-lg p-2 border border-red-500/20">
                <span class="text-xs text-red-300">HP</span>
                <p class="font-bold text-red-200">{{ parseFloat(beatmap.diff_drain).toFixed(1) }}</p>
              </div>
            </div>

            <!-- Play Stats -->
            <div class="grid grid-cols-3 gap-3 mb-4">
              <div class="bg-white/5 rounded-lg p-3">
                <div class="flex items-center gap-2 mb-1">
                  <Play class="w-4 h-4 text-gray-400" />
                  <span class="text-xs text-gray-400">Plays</span>
                </div>
                <p class="font-semibold">{{ formatNumber(beatmap.playcount) }}</p>
              </div>

              <div class="bg-white/5 rounded-lg p-3">
                <div class="flex items-center gap-2 mb-1">
                  <CheckCircle class="w-4 h-4 text-gray-400" />
                  <span class="text-xs text-gray-400">Passes</span>
                </div>
                <p class="font-semibold">{{ formatNumber(beatmap.passcount) }}</p>
              </div>

              <div class="bg-white/5 rounded-lg p-3">
                <div class="flex items-center gap-2 mb-1">
                  <Percent class="w-4 h-4 text-gray-400" />
                  <span class="text-xs text-gray-400">Pass Rate</span>
                </div>
                <p class="font-semibold">{{ calculatePassRate(beatmap.playcount, beatmap.passcount) }}%</p>
              </div>
            </div>

            <!-- Object Count -->
            <div class="flex items-center gap-4 mb-4 text-sm">
              <div class="flex items-center gap-2">
                <div class="w-3 h-3 rounded-full bg-pink-400"></div>
                <span class="text-gray-300">Circles: <span class="font-semibold text-white">{{ beatmap.count_normal }}</span></span>
              </div>
              <div class="flex items-center gap-2">
                <div class="w-3 h-3 rounded-full bg-blue-400"></div>
                <span class="text-gray-300">Sliders: <span class="font-semibold text-white">{{ beatmap.count_slider }}</span></span>
              </div>
              <div class="flex items-center gap-2">
                <div class="w-3 h-3 rounded-full bg-purple-400"></div>
                <span class="text-gray-300">Spinners: <span class="font-semibold text-white">{{ beatmap.count_spinner }}</span></span>
              </div>
            </div>

            <!-- Tags -->
            <div v-if="beatmap.tags" class="mb-4">
              <div class="flex flex-wrap gap-2">
                <span
                  v-for="tag in beatmap.tags.split(' ').slice(0, 10)"
                  :key="tag"
                  class="px-2 py-1 bg-white/5 text-gray-300 rounded text-xs"
                >
                  {{ tag }}
                </span>
              </div>
            </div>

            <!-- Footer Info -->
            <div class="flex items-center gap-2 text-sm text-gray-400 flex-wrap">
              <Hash class="w-4 h-4" />
              <span>ID: {{ beatmap.beatmap_id }}</span>
              <span class="text-gray-600">•</span>
              <span>Set: {{ beatmap.beatmapset_id }}</span>
              <span class="text-gray-600">•</span>
              <Calendar class="w-4 h-4" />
              <span>{{ formatDate(beatmap.approved_date) }}</span>
              
              <div class="ml-auto flex items-center gap-2">
                <Film v-if="beatmap.video === '1'" class="w-4 h-4 text-pink-400" title="Has Video" />
                <FileText v-if="beatmap.storyboard === '1'" class="w-4 h-4 text-purple-400" title="Has Storyboard" />
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- No Results -->
      <div v-else-if="!loading && searched" class="text-center py-20">
        <FileQuestion class="w-16 h-16 text-gray-500 mx-auto mb-4" />
        <h3 class="text-xl font-semibold text-gray-400 mb-2">No beatmaps found</h3>
        <p class="text-gray-500">Try adjusting your search parameters</p>
      </div>

      <!-- Initial State -->
      <div v-else class="text-center py-20">
        <Search class="w-16 h-16 text-gray-500 mx-auto mb-4" />
        <h3 class="text-xl font-semibold text-gray-400 mb-2">Ready to search</h3>
        <p class="text-gray-500">Enter your search criteria above to find beatmaps</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';
import { OsynicOsuApiV1GlooClient } from '@osynicite/osynic-osuapi';
import type { GetBeatmapsParams, BeatmapV1 } from '@osynicite/osynic-osuapi';
import {
  Search,
  Music,
  Hash,
  User,
  Layers,
  Gamepad2,
  ListFilter,
  Sparkles,
  X,
  AlertCircle,
  Loader2,
  FileMusic,
  Clock,
  Target,
  Heart,
  Calendar,
  FileQuestion,
  Key,
  ToggleLeft,
  Filter,
  Play,
  CheckCircle,
  Percent,
  Film,
  FileText
} from 'lucide-vue-next';

// Initialize client
const client = new OsynicOsuApiV1GlooClient("YOUR_API_KEY_HERE"); // Replace with your osu! API v1 key
client.setProxyUrl("YOUR_PROXY_URL_HERE"); // You can see https://github.com/Islatri/deno_osynic_cors

// State
const searchParams = reactive<Partial<GetBeatmapsParams>>({
  api_key: null,
  since: null,
  sid: null,
  bid: null,
  uid: null,
  typee: null,
  mode: null,
  has_converted: null,
  hash: null,
  limit: 10,
  mods: null
});

const beatmaps = ref<BeatmapV1[]>([]);
const loading = ref(false);
const error = ref<string | null>(null);
const searched = ref(false);

// Methods
const searchBeatmaps = async () => {
  loading.value = true;
  error.value = null;
  searched.value = true;

  try {
    // Filter out null/empty values
    const params: any = {};
    Object.keys(searchParams).forEach(key => {
      const value = searchParams[key as keyof typeof searchParams];
      if (value !== '' && value !== null && value !== undefined) {
        params[key] = value;
      }
    });

    const result = await client.getBeatmaps(params);
    beatmaps.value = Array.isArray(result) ? result : [result];
  } catch (err: any) {
    error.value = err?.message ?? String(err);
    beatmaps.value = [];
  } finally {
    loading.value = false;
  }
};

const clearSearch = () => {
  searchParams.api_key = null;
  searchParams.since = null;
  searchParams.sid = null;
  searchParams.bid = null;
  searchParams.uid = null;
  searchParams.typee = null;
  searchParams.mode = null;
  searchParams.has_converted = null;
  searchParams.hash = null;
  searchParams.limit = 10;
  searchParams.mods = null;
  
  beatmaps.value = [];
  error.value = null;
  searched.value = false;
};

const getDifficultyColor = (rating: string): string => {
  const diff = Number.parseFloat(rating);
  if (diff < 2) return 'bg-blue-500/30 text-blue-200';
  if (diff < 2.7) return 'bg-green-500/30 text-green-200';
  if (diff < 4) return 'bg-yellow-500/30 text-yellow-200';
  if (diff < 5.3) return 'bg-orange-500/30 text-orange-200';
  if (diff < 6.5) return 'bg-red-500/30 text-red-200';
  return 'bg-purple-500/30 text-purple-200';
};

const getModeName = (mode: string): string => {
  const modeMap: Record<string, string> = {
    '0': 'osu!',
    '1': 'Taiko',
    '2': 'Catch',
    '3': 'Mania'
  };
  return modeMap[mode] ?? 'Unknown';
};

const getApprovedStatus = (approved: string): string => {
  const statusMap: Record<string, string> = {
    '4': 'Loved',
    '3': 'Qualified',
    '2': 'Approved',
    '1': 'Ranked',
    '0': 'Pending',
    '-1': 'WIP',
    '-2': 'Graveyard'
  };
  return statusMap[approved] ?? 'Unknown';
};

const getApprovedColor = (approved: string): string => {
  const colorMap: Record<string, string> = {
    '4': 'bg-pink-500/30 text-pink-200',
    '3': 'bg-cyan-500/30 text-cyan-200',
    '2': 'bg-green-500/30 text-green-200',
    '1': 'bg-blue-500/30 text-blue-200',
    '0': 'bg-yellow-500/30 text-yellow-200',
    '-1': 'bg-orange-500/30 text-orange-200',
    '-2': 'bg-gray-500/30 text-gray-200'
  };
  return colorMap[approved] ?? 'bg-gray-500/30 text-gray-200';
};

const formatDuration = (seconds: string): string => {
  const total = Number.parseInt(seconds);
  const mins = Math.floor(total / 60);
  const secs = total % 60;
  return `${mins}:${secs.toString().padStart(2, '0')}`;
};

const formatDate = (dateStr: string | null): string => {
  if (!dateStr) return 'N/A';
  const date = new Date(dateStr);
  return date.toLocaleDateString('en-US', { year: 'numeric', month: 'short', day: 'numeric' });
};

const formatNumber = (num: string): string => {
  return Number.parseInt(num).toLocaleString();
};

const calculatePassRate = (playcount: string, passcount: string): string => {
  const plays = Number.parseInt(playcount);
  const passes = Number.parseInt(passcount);
  if (plays === 0) return '0';
  return ((passes / plays) * 100).toFixed(1);
};
</script>