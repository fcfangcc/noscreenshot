<script lang="ts" setup>
import { reactive, onMounted, ref } from 'vue'
let accessStatus = reactive({ icon: 'info', title: 'Unknow', subTitle: 'Click TEST' })
let loading = ref(false)

// eslint-disable-next-line @typescript-eslint/no-empty-function
onMounted(() => {})

const testAccessStatus = async (): Promise<boolean> => {
  // todo: invoke system call
  return accessStatus.icon === 'error'
}

const changeToSuccess = () => {
  accessStatus.icon = 'success'
  accessStatus.title = 'Success'
  accessStatus.subTitle = 'Access granted'
}

const changeToFailure = () => {
  accessStatus.icon = 'error'
  accessStatus.title = 'Error'
  accessStatus.subTitle = 'Access denied'
}

const testAccess = async () => {
  loading.value = true
  if (await testAccessStatus()) {
    changeToSuccess()
  } else {
    changeToFailure()
  }
  loading.value = false
}
</script>

<template>
  <div>
    <div class="helper-doc">
      <div>1.选取苹果菜单 >“系统设置”，然后在边栏中点按“隐私与安全性” 。（你可能需要向下滚动。）</div>
      <div>2.点按“屏幕录制”。</div>
      <div>3.在列表中为每个 App 打开或关闭屏幕录制。</div>
    </div>

    <div>
      <el-result :icon="accessStatus.icon" :title="accessStatus.title" :sub-title="accessStatus.subTitle">
        <template #extra>
          <el-button type="primary" @click="testAccess" :loading="loading">Test</el-button>
        </template>
      </el-result>
    </div>
  </div>
</template>

<style scoped>
.helper-doc {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  width: 50%;
  margin: 50px auto 0 auto;
}
</style>
