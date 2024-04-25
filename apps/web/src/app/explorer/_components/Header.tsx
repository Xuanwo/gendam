'use client'
import { useExplorerContext } from '@/Explorer/hooks'
import PageNav from '@/components/PageNav'
import UploadButton from '@/components/UploadButton'
import Viewport from '@/components/Viewport'
import { rspc } from '@/lib/rspc'
import { useUploadQueueStore } from '@/store/uploadQueue'
import Icon from '@muse/ui/icons'
import classNames from 'classnames'
import { useRouter } from 'next/navigation'
import { useCallback, useEffect, useState } from 'react'
import SearchForm from '../../search/SearchForm'  // TODO: 这样不大好，应该是一个公共组件
import { useInspector } from './Inspector'
import TitleDialog, { useTitleDialog } from './TitleDialog'
import { Button } from '@muse/ui/v2/button'

export default function Header() {
  const titleDialog = useTitleDialog()
  const router = useRouter()
  const explorer = useExplorerContext()
  const uploadQueueStore = useUploadQueueStore()

  const inspector = useInspector()

  const handleSelectFiles = useCallback(
    (fileFullPaths: string[]) => {
      if (explorer.materializedPath) {
        for (const fileFullPath of fileFullPaths) {
          const name = fileFullPath.split('/').slice(-1).join('')
          uploadQueueStore.enqueue({
            materializedPath: explorer.materializedPath,
            name: name,
            localFullPath: fileFullPath,
          })
        }
      }
    },
    [explorer.materializedPath, uploadQueueStore],
  )

  const handleSearch = useCallback((text: string, recordType: string) => {
    const search = new URLSearchParams()
    search.set('text', text)
    search.set('recordType', recordType)
    router.push(`/search?${search}`)
  }, [router])

  useEffect(() => {
    if (typeof window !== 'undefined' && typeof window.__TAURI__ !== 'undefined') {
      let unlisten: () => void
      let isExit = false
      import('@tauri-apps/api/event').then(async ({ listen }) => {
        if (isExit) {
          return
        }
        unlisten = await listen('tauri://file-drop', (event) => {
          const files = event.payload as string[]
          console.log('files dropped', files)
          handleSelectFiles(files)
        })
      })
      return () => {
        isExit = true
        if (unlisten) {
          unlisten()
        }
      }
    }
  }, [handleSelectFiles])

  return (
    <>
      <Viewport.Toolbar className="justify-start">
        <PageNav
          title={explorer.materializedPath === '/' ? 'Home' : explorer.materializedPath}
          className="w-1/3"
        />
        <div className="w-1/3">
          <SearchForm
            initialSearchPayload={null}
            onSubmit={(text: string, recordType: string) => handleSearch(text, recordType)}
          />
        </div>
        <div className="ml-auto"></div>
        <div className="text-ink/70 flex items-center gap-1 justify-self-end">
          <Button variant="ghost" size="sm" className="h-7 w-7 p-1 transition-none" onClick={() => titleDialog.setOpen(true)}>
            <Icon.FolderAdd className="size-4" />
          </Button>
          <Button variant="ghost" size="sm" className="h-7 w-7 p-1 transition-none" asChild>
            {/* 加上 asChild 不使用 native button, 因为里面是个 form, native button 可能会触发 form submit */}
            <UploadButton onSelectFiles={handleSelectFiles}>
              <Icon.Upload className="size-4" />
            </UploadButton>
          </Button>

          <div className="bg-toolbar-line mx-1 h-4 w-px"></div>

          <Button
            variant="ghost" size="sm"
            className={classNames('h-7 w-7 p-1 transition-none', explorer.settings.layout === 'grid' && 'bg-toolbar-hover')}
            onClick={() => explorer.settings.update({ layout: 'grid' })}
          >
            <Icon.Grid className="size-4" />
          </Button>
          <Button
            variant="ghost" size="sm"
            className={classNames('h-7 w-7 p-1 transition-none', explorer.settings.layout === 'list' && 'bg-toolbar-hover')}
            onClick={() => explorer.settings.update({ layout: 'list' })}
          >
            <Icon.List className="size-4" />
          </Button>
          <Button
            variant="ghost" size="sm"
            className={classNames('h-7 w-7 p-1 transition-none', explorer.settings.layout === 'media' && 'bg-toolbar-hover')}
            onClick={() => explorer.settings.update({ layout: 'media' })}
          >
            <Icon.SelfAdapting className="size-4" />
          </Button>

          <div className="bg-toolbar-line mx-1 h-4 w-px"></div>

          <Button
            variant="ghost" size="sm"
            className={classNames('h-7 w-7 p-1 transition-none', inspector.show && 'bg-toolbar-hover')}
            onClick={() => inspector.setShow(!inspector.show)}
          >
            <Icon.Sidebar className="size-4" />
          </Button>
        </div>
      </Viewport.Toolbar>
      <TitleDialog />
    </>
  )
}
