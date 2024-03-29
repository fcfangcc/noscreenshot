<script setup lang="ts">
import chiModelUrl from '@/assets/chi_sim.traineddata?url'
import { logger } from '@/command'
import { Picture as IconPicture } from '@element-plus/icons-vue'
import { invoke } from '@tauri-apps/api'
import { message, confirm } from '@tauri-apps/api/dialog'
import { listen } from '@tauri-apps/api/event'
import { platform } from '@tauri-apps/api/os'
import { writeText } from '@tauri-apps/api/clipboard'
import { isRegistered, register } from '@tauri-apps/api/globalShortcut'
import { appWindow, availableMonitors, getAll } from '@tauri-apps/api/window'
import { OCRClient } from 'tesseract-wasm'
import { onMounted, reactive, ref } from 'vue'
import { ElMessage } from 'element-plus'
import { useI18n } from 'vue-i18n'
import { screenCaptureAccess, openWindow, config } from '@/apis/index'
import { attachConsole } from 'tauri-plugin-log-api'

const { t } = useI18n()

let screenshotImg = ref('')
let uniqId = ref('')
let screenshotWindows = reactive<string[]>([])
let unlistens: any = []
let ocrData = ref('')
let ocrLoading = ref(false)
let screenshotLoading = ref(false)
let srcList = reactive<string[]>([])

const sleep = (delay: number) => new Promise((resolve) => setTimeout(resolve, delay))

onMounted(async () => {
  console.log(await config.getConfig())
  await attachConsole()
  await registerShortcut()
  // only request once.
  if ((await screenCaptureAccess()) === false) {
    await screenCaptureAccess(true)
  }

  await appWindow.onCloseRequested(async (event) => {
    event.preventDefault()

    for (let w of getAll().filter((i) => screenshotWindows.includes(i.label))) {
      try {
        await w.close()
      } catch (e) {
        break
      }
    }
  })
  // 监听截图成功方法
  unlistens.push(
    await listen<string>('screenshotOk', async (event) => {
      screenshotLoading.value = false
      const notMainWindows = getAll().filter((i) => i.label !== 'main')

      screenshotImg.value = event.payload
      srcList.unshift(event.payload)
      for (let w of notMainWindows) {
        await w.close()
      }

      if (!(await appWindow.isVisible())) {
        await appWindow.show()
      }
      await appWindow.unminimize()
      screenshotWindows = []
      await clearTempDir()
      await ocr(event.payload)
    })
  )
  // 退出截图。由于close方法有bug，这里继续event主窗口来实现
  unlistens.push(
    await listen<string>('window-esc', async () => {
      screenshotLoading.value = false
      const notMainWindows = getAll().filter((i) => i.label !== 'main')
      for (let w of notMainWindows) {
        await w.close()
      }
      screenshotWindows = []
      await clearTempDir()
    })
  )
})

const clearTempDir = async () => {
  await invoke('clear_temp', { id: uniqId.value })
}

const screenshot = async () => {
  if ((await screenCaptureAccess()) === false) {
    const confirmed = await confirm(t('message.accessDenied'), 'ERROR')
    if (confirmed) {
      await openWindow()
    }
    return
  }

  if (screenshotLoading.value || ocrLoading.value) {
    ElMessage({
      message: t('message.doubleTask'),
      type: 'error',
      showClose: true,
      duration: 0
    })
    return
  }
  screenshotLoading.value = true

  const platformName = await platform()

  if (!(await appWindow.isMinimized()) && platformName !== 'darwin') {
    await appWindow.minimize()
    await sleep(500)
  }
  try {
    ocrData.value = ''
    uniqId.value = Date.now().toString()
    let [schema, images] = await invoke<string[]>('screenshot', {
      id: uniqId.value
    })
    let monitors = await availableMonitors()

    for (let [index, m] of monitors.entries()) {
      // 这里顺序可能有bug，出现对不上的情况
      const logical = m.size.toLogical(m.scaleFactor)

      await showScreenshotWindow(index.toString(), schema, images[index], {
        x: m.position.x,
        y: m.position.y,
        height: logical.height,
        width: logical.width
      })
    }
  } catch (e) {
    console.error(e)
    screenshotLoading.value = false
  }
}

const registerShortcut = async () => {
  const key = 'CommandOrControl+Shift+Z'
  const registered = await isRegistered(key)
  if (registered) {
    return
  }

  try {
    await register(key, async () => {
      await screenshot()
    })
  } catch (e: any) {
    const msg = `registerShortcut ${key} failed. ${e}`
    logger.error(msg)
    await message(msg, { title: 'register error', type: 'error' })
  }
}

const showScreenshotWindow = async (name: string, schema: string, image: string, args: any) => {
  try {
    let label: string = await invoke('show_screenshot', {
      info: {
        url: `/screenshot?schema=${schema}&image=${image}`,
        label: `theUniqueLabel${name}`,
        ...args
      }
    })
    screenshotWindows.push(label)
  } catch (e: any) {
    logger.error(e)
    await message(e, { title: 'open window error', type: 'error' })
  }
}

const ocr = async (b64data: string) => {
  const imageResponse = await fetch(b64data)
  const imageBlob = await imageResponse.blob()
  const image = await createImageBitmap(imageBlob)

  const ocr = new OCRClient()
  ocrLoading.value = true
  try {
    // Load the appropriate OCR training data for the image(s) we want to
    // process.
    await ocr.loadModel(chiModelUrl)

    await ocr.loadImage(image)

    // Perform text recognition and return text in reading order.
    const text = await ocr.getText()
    ocrData.value = text
  } finally {
    // Once all OCR-ing has been done, shut down the Web Worker and free up
    // resources.
    ocrLoading.value = false
    ocr.destroy()
  }
}

const clipboard = async () => {
  if (ocrData.value) {
    await writeText(ocrData.value)
    ElMessage({
      message: t('message.clipboard'),
      type: 'success'
    })
  }
}
</script>

<template>
  <div class="container">
    <router-view />
    <h1>Welcome to noScreenshot!</h1>
    <div class="gen-div">
      <el-button type="primary" @click="screenshot()" :loading="ocrLoading">{{ t('screenshot') }}</el-button>
      <el-button type="primary" :disabled="!ocrData" @click="clipboard()">{{ t('clipboard') }}</el-button>
    </div>
    <div class="gen-div">
      <el-input
        v-model="ocrData"
        :autosize="{ minRows: 4, maxRows: 10 }"
        type="textarea"
        class="ocr-data"
        v-loading="ocrLoading"
        element-loading-text="analysis..."
        element-loading-background="rgba(122, 122, 122, 0.8)"
      />
    </div>

    <div>
      <el-image class="el-image-inner" :src="screenshotImg" :preview-src-list="srcList" fit="none">
        <template #error>
          <div class="image-slot">
            <el-icon><icon-picture /></el-icon>
          </div>
        </template>
      </el-image>
    </div>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

.gen-div {
  margin-bottom: 20px;
}

.ocr-data {
  width: 60%;
}

.el-image-inner {
  padding: 0 5px;
  max-width: 60%;
  max-height: 300px;
  width: 100%;
  height: 200px;
}

.image-slot {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 100%;
  height: 100%;
  background: #d1dbe5;
  color: var(--el-text-color-secondary);
  font-size: 30px;
}
</style>
