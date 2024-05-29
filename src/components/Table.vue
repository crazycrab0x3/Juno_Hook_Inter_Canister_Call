<script lang="ts" setup>
import { type Doc, listDocs } from '@junobuild/core'
import { onMounted, onUnmounted, ref } from 'vue'
import type { UpdateNote } from '@/types/note'
import Delete from '@/components/Delete.vue'

const items = ref<Doc<UpdateNote>[]>([])

const list = async () => {
  const { items: data } = await listDocs<UpdateNote>({
    collection: 'notes',
    filter: {}
  })

  items.value = data
}

onMounted(async () => {
  window.addEventListener('reload', list)
  await list()
})

onUnmounted(() => window.removeEventListener('reload', list))
</script>

<template>
  <div class="w-full max-w-3xl mt-8 dark:text-white" role="table">
    <div role="row">
      <span role="columnheader" aria-sort="none"> Entries </span>
    </div>

    <div class="py-2" role="rowgroup">
      <div
        class="grid grid-cols-12 items-center gap-2 px-3 mb-4 border-black dark:border-lavender-blue-500 border-[3px] rounded bg-white dark:bg-black dark:text-white transition-all shadow-[8px_8px_0px_rgba(0,0,0,1)] dark:shadow-[8px_8px_0px_#7888FF]"
        role="row"
      >
        <span role="cell" aria-rowindex="{index}" class="p-1 flex align-center min-w-max col-span-1">No</span>
        <div role="cell" class="line-clamp-3 overflow-hidden grow col-span-3">Product Name</div>
        <div role="cell" class="line-clamp-3 overflow-hidden grow col-span-2">Price In Cents</div>
        <div role="cell" class="line-clamp-3 overflow-hidden grow col-span-2">Currency</div>
        <div role="cell" class="line-clamp-3 overflow-hidden grow col-span-3">USD Price In Cents</div>
        <div role="cell" class="flex gap-2 justify-center align-middle col-span-1">Action</div>
      </div>
      <div
        v-bind:key="index"
        class="grid grid-cols-12 items-center gap-2 px-3 mb-4 border-black dark:border-lavender-blue-500 border-[3px] rounded bg-white dark:bg-black dark:text-white transition-all shadow-[8px_8px_0px_rgba(0,0,0,1)] dark:shadow-[8px_8px_0px_#7888FF]"
        role="row"
        v-for="(item, index) in items"
      >
        <span role="cell" aria-rowindex="{index}" class="p-1 flex align-center min-w-max col-span-1">{{ index + 1 }}</span>
        <div role="cell" class="line-clamp-3 overflow-hidden grow col-span-3">{{ item.data.name }}</div>
        <div role="cell" class="line-clamp-3 overflow-hidden grow col-span-2">{{ item.data.price_cents }}</div>
        <div role="cell" class="line-clamp-3 overflow-hidden grow col-span-2">{{ item.data.currency }}</div>
        <div role="cell" class="line-clamp-3 overflow-hidden grow col-span-3">{{ item.data.price_usd_cents }}</div>
        <div role="cell" class="flex gap-2 justify-center align-middle col-span-1">
          <Delete :doc="item" :reload="list" />
        </div>
      </div>
    </div>
  </div>
</template>
