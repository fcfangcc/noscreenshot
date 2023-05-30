import { createI18n } from 'vue-i18n'

const messages = {
  en: {
    screenshot: 'Screenshot',
    clipboard: 'Clipboard',
    message: {
      clipboard: 'Copy to clipboard success.',
      doubleTask: 'There are already unfinished task!'
    },
    helper: {
      SCAccess: [
        '1.Choose Apple menu > System Settings, then click Privacy & Security in the sidebar. (You may need to scroll down.)',
        '2.Open Privacy & Security settings for me.Click Screen Recording.',
        '3.Turn screen recording on or off for each app in the list.'
      ]
    },
    testAccess: 'TestAccess'
  },
  zh: {
    screenshot: '截图',
    clipboard: '拷贝内容',
    message: {
      clipboard: '内容已复制到黏贴板',
      doubleTask: '已经有未完成的任务'
    },
    helper: {
      SCAccess: [
        '1.选取苹果菜单 >“系统设置”，然后在边栏中点按“隐私与安全性” 。（你可能需要向下滚动。）',
        '2.点按“屏幕录制”。',
        '3.在列表中为每个 App 打开或关闭屏幕录制。'
      ]
    },
    testAccess: '权限测试'
  }
}
const i18n = createI18n({
  legacy: false,
  locale: 'en',
  messages
})

export default i18n
