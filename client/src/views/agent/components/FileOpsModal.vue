<template>
  <a-modal
    :visible="visible"
    @update:visible="$emit('update:visible', $event)"
    title="文件管理"
    :footer="null"
    width="520"
    destroyOnClose
  >
    <a-tabs v-model:activeKey="activeTab">
      <!-- Upload Tab -->
      <a-tab-pane key="upload" tab="上传文件">
        <a-form layout="vertical" class="mt-4">
          <a-form-item label="远程路径">
            <a-input v-model:value="uploadForm.remotePath" placeholder="例如: C:\Users\target\payload.exe 或 /tmp/payload" />
          </a-form-item>
          <a-form-item label="选择文件">
            <input
              type="file"
              class="block w-full text-sm text-slate-500
                file:mr-3 file:py-1.5 file:px-3
                file:rounded-md file:border-0
                file:text-sm file:font-medium
                file:bg-blue-50 file:text-blue-600
                hover:file:bg-blue-100
                cursor-pointer"
              @change="onFileSelect"
            />
          </a-form-item>
          <a-form-item v-if="uploadForm.selectedFile" :label="'已选择: ' + uploadForm.selectedFile.name + ' (' + formatSize(uploadForm.selectedFile.size) + ')'">
          </a-form-item>
          <a-form-item>
            <a-button type="primary" :loading="uploading" :disabled="!uploadForm.selectedFile || !uploadForm.remotePath" @click="doUpload">
              上传到目标
            </a-button>
          </a-form-item>
        </a-form>
      </a-tab-pane>

      <!-- Download Tab -->
      <a-tab-pane key="download" tab="下载文件">
        <a-form layout="vertical" class="mt-4">
          <a-form-item label="远程文件路径">
            <a-input v-model:value="downloadForm.remotePath" placeholder="例如: C:\Users\target\documents\secret.docx 或 /etc/passwd" />
          </a-form-item>
          <a-form-item>
            <a-button type="primary" :loading="downloading" :disabled="!downloadForm.remotePath" @click="doDownload">
              从目标下载
            </a-button>
          </a-form-item>
        </a-form>
      </a-tab-pane>
    </a-tabs>
  </a-modal>
</template>

<script setup lang="ts">
import { ref, reactive } from 'vue';
import { message } from '@/utils/message';
import { uploadFile, downloadFile } from '@/api/agent';
import type { Agent } from '@/api/agent';
import { useEventStore } from '@/store/events';

const props = defineProps<{
  visible: boolean;
  agent: Agent | null;
}>();

const emit = defineEmits(['update:visible']);

const eventStore = useEventStore();

const activeTab = ref('upload');
const uploading = ref(false);
const downloading = ref(false);

const uploadForm = reactive({
  remotePath: '',
  selectedFile: null as File | null,
  fileBase64: '',
});

const downloadForm = reactive({
  remotePath: '',
});

function onFileSelect(e: Event) {
  const target = e.target as HTMLInputElement;
  const file = target.files?.[0];
  if (!file) return;

  if (file.size > 10 * 1024 * 1024) {
    message.warning('文件大小不能超过 10MB');
    return;
  }

  uploadForm.selectedFile = file;
  const reader = new FileReader();
  reader.onload = () => {
    const result = reader.result as string;
    uploadForm.fileBase64 = result.split(',')[1] || '';
  };
  reader.readAsDataURL(file);
}

async function doUpload() {
  if (!props.agent || !uploadForm.selectedFile || !uploadForm.remotePath) return;

  uploading.value = true;
  try {
    const res = await uploadFile(props.agent.agent_id, uploadForm.remotePath, uploadForm.fileBase64);
    if (res.success) {
      message.success(`上传任务已下发 (task: ${res.task_id || '-'})`);
      uploadForm.remotePath = '';
      uploadForm.selectedFile = null;
      uploadForm.fileBase64 = '';
    } else {
      message.error(res.detail || '上传失败');
    }
  } catch (e: any) {
    message.error(e.message);
  } finally {
    uploading.value = false;
  }
}

async function doDownload() {
  if (!props.agent || !downloadForm.remotePath) return;

  downloading.value = true;
  try {
    const res = await downloadFile(props.agent.agent_id, downloadForm.remotePath);
    if (res.success) {
      if (res.task_id) {
        const fileName = downloadForm.remotePath.split(/[/\\]/).pop() || 'download';
        eventStore.registerDownload(res.task_id, fileName);
      }
      message.success(`下载任务已下发，文件将自动下载`);
      downloadForm.remotePath = '';
    } else {
      message.error(res.detail || '下载失败');
    }
  } catch (e: any) {
    message.error(e.message);
  } finally {
    downloading.value = false;
  }
}

function formatSize(bytes: number): string {
  if (bytes < 1024) return `${bytes} B`;
  if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
  return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
}
</script>
