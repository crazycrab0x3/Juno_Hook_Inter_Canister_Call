<script lang="ts" setup>
import { computed, ref } from 'vue'
import { useAuthStore } from '@/stores/auth.store'
import { storeToRefs } from 'pinia'
import { setDoc, uploadFile } from '@junobuild/core'
import { nanoid } from 'nanoid'
import type { Note } from '@/types/note'
import Button from '@/components/Button.vue'
import Backdrop from '@/components/Backdrop.vue'

const showModal = ref(false)
const inputText = ref('')
const inputFile = ref<HTMLInputElement | null>(null)
const progress = ref(false)
const file = ref<File | undefined>(undefined)
const productName = ref<string>("");
const currency = ref<string>("JPY");
const price = ref<number>(0);


const store = useAuthStore()
const { user } = storeToRefs(store)

const valid = computed(() => inputText.value !== '' && user !== undefined && user !== null)

const setShowModal = (value: boolean) => (showModal.value = value)
const setFile = (f: File | undefined) => (file.value = f)

const reload = () => {
  let event = new Event('reload')
  window.dispatchEvent(event)
}

const add = async () => {
  // Demo purpose therefore edge case not properly handled
  if (user.value === null || user.value === undefined) {
    return
  }

  progress.value = true

  try {

    const key = nanoid()

    await setDoc<Note>({
      collection: 'notes',
      doc: {
        key,
        data: {
          name: productName.value,
          price_cents: price.value,
          currency: currency.value
        }
      }
    })
    productName.value = "";
    price.value = 0;
    currency.value = "";
    setShowModal(false)

    reload()
  } catch (err) {
    console.error(err)
  }

  progress.value = false
}

</script>

<template>
  <Button @click="() => setShowModal(true)">
    Add an entry
    <svg
      xmlns="http://www.w3.org/2000/svg"
      height="20"
      viewBox="0 -960 960 960"
      width="20"
      fill="currentColor"
    >
      <path d="M417-417H166v-126h251v-251h126v251h251v126H543v251H417v-251Z" />
    </svg>
  </Button>

  <div
    class="fixed inset-0 z-50 p-16 md:px-24 md:py-44 animate-fade"
    role="dialog"
    v-if="showModal"
  >
    <div class="relative w-full max-w-xl">
      <input
        class="mb-2 form-control block w-full px-3 py-1.5 text-base font-normal m-0 resize-none border-black border-[3px] rounded-sm bg-white shadow-[5px_5px_0px_rgba(0,0,0,1)] focus:outline-none"
        placeholder="Product Name"
        v-model="productName"
        :disabled="progress"
      ></input>
      <input
        class="mb-2 form-control block w-full px-3 py-1.5 text-base font-normal m-0 resize-none border-black border-[3px] rounded-sm bg-white shadow-[5px_5px_0px_rgba(0,0,0,1)] focus:outline-none"
        placeholder="Price in Cents"
        v-model="price" 
        type="number"
        :disabled="progress"
      ></input>
      <select
        class="mb-2 form-control block w-full px-3 py-1.5 text-base font-normal m-0 resize-none border-black border-[3px] rounded-sm bg-white shadow-[5px_5px_0px_rgba(0,0,0,1)] focus:outline-none"
        v-model="currency"
        :disabled="progress"
      >
        <option value="JPY">JPY</option>
        <option value="IDR">IDR</option>
        <option value="CHF">CHF</option>
        <option value="GBP">GBP</option>
        <option value="EUR">EUR</option>
        <option value="CAD">CAD</option>
        <option value="USD">USD</option>
      </select>


        <div class="flex my-4" v-if="!progress">
          <button
            class="py-1 px-8 hover:text-lavender-blue-600 active:text-lavender-blue-400"
            type="button"
            @click="() => setShowModal(false)"
          >
            Close
          </button>

          <Button @click="add"> Submit </Button>
        </div>
    </div>
  </div>
  <Backdrop v-if="showModal" :spinner="false" />
</template>
