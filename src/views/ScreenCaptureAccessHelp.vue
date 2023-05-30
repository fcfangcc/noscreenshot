<script lang="ts" setup>
import { reactive, onMounted, ref } from 'vue'
import { useI18n } from 'vue-i18n'

const { t } = useI18n()

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
      <div>{{ t('helper.SCAccess.0') }}</div>
      <div>{{ t('helper.SCAccess.1') }}</div>
      <div>{{ t('helper.SCAccess.2') }}</div>
    </div>

    <div>
      <el-result :icon="accessStatus.icon" :title="accessStatus.title" :sub-title="accessStatus.subTitle">
        <template #extra>
          <el-button type="primary" @click="testAccess" :loading="loading">{{ t('testAccess') }}</el-button>
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
