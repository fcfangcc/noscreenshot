import { invoke } from '@tauri-apps/api'
import { trace, info, error, warn } from 'tauri-plugin-log-api'

export const listFormats = async (): Promise<string[]> => {
  return await invoke('list_formats')
}

export const logger = { error, info, warn, trace }
