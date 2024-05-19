'use client'
import { ContextMenu } from '@gendam/ui/v2/context-menu'
import { useQuickViewStore } from '@/components/Shared/QuickView/store'
import { useCurrentLibrary } from '@/lib/library'
import { formatDuration } from '@/lib/utils'
import classNames from 'classnames'
import Image from 'next/image'
import { useCallback, useEffect, useRef } from 'react'
import { type ItemsWithSize } from './SearchResults'
import { useRouter } from 'next/navigation'

const VideoItem: React.FC<{ item: ItemsWithSize }> = ({ item }) => {
  const router = useRouter()
  const quickViewStore = useQuickViewStore()
  const currentLibrary = useCurrentLibrary()
  const videoRef = useRef<HTMLVideoElement>(null)

  useEffect(() => {
    const video = videoRef.current
    if (!video) return
    let startTime = Math.max(0, item.metadata.startTime / 1e3 - 0.5)
    let endTime = Math.max(startTime, item.metadata.endTime / 1e3 + 1.5)
    video.currentTime = startTime
    video.ontimeupdate = () => {
      if (video.currentTime >= endTime) {
        // video.pause();
        // video.ontimeupdate = null;
        video.currentTime = startTime
      }
    }
  }, [item.metadata])

  const quickview = useCallback(
    () => {
      quickViewStore.open({
        name: item.filePath.name,
        assetObject: item.filePath.assetObject!,
        video: {
          currentTime: item.metadata.startTime / 1e3,
        },
      })
    },
    [quickViewStore, item.filePath, item.metadata],
  )

  const reveal = useCallback(() => {
    router.push(`/explorer?dir=${item.filePath.materializedPath}&id=${item.filePath.id}`)
  }, [item.filePath, router])

  return (
    <ContextMenu.Root>
      <ContextMenu.Trigger>
        <div
          className={classNames('border-app-line/75 group relative overflow-hidden rounded-xl border-4')}
          // style={{ minWidth: `${width}rem`, height: '10rem', flex: frames.length }}
          style={{ width: `${item.width}px`, height: `${item.height}px` }}
          onClick={() => quickview()}
        >
          <div className="flex h-full items-stretch justify-between">
            {item.frames.map((frame, index) => (
              <div key={index} className="visible relative flex-1 cursor-pointer bg-neutral-100">
                <Image
                  src={currentLibrary.getThumbnailSrc(item.filePath.assetObject?.hash!, frame)}
                  alt={item.filePath.name}
                  fill={true}
                  className="object-cover"
                  priority
                ></Image>
              </div>
            ))}
          </div>
          <div
            className={classNames(
              'absolute left-0 top-0 flex h-full w-full flex-col justify-between bg-black/60 px-4 py-2 text-neutral-300',
              'invisible group-hover:visible',
            )}
          >
            <div className="truncate text-xs">
              {item.filePath.materializedPath}
              {item.filePath.name}
            </div>
            <div className="flex items-center justify-between text-xs">
              <div>{formatDuration(item.metadata.startTime / 1000)}</div>
              <div>→</div>
              <div>{formatDuration(item.metadata.endTime / 1000 + 1)}</div>
            </div>
          </div>
        </div>
      </ContextMenu.Trigger>
      <ContextMenu.Portal>
        <ContextMenu.Content onClick={(e) => e.stopPropagation()}>
          <ContextMenu.Item onSelect={() => quickview()}>
            <div>Quick view</div>
          </ContextMenu.Item>
          <ContextMenu.Item onSelect={() => reveal()}>
            <div>Reveal in explorer</div>
          </ContextMenu.Item>
        </ContextMenu.Content>
      </ContextMenu.Portal>
    </ContextMenu.Root>
  )
}

export default VideoItem
