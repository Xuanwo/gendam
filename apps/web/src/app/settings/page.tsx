'use client'
import PageNav from '@/components/PageNav'
import Viewport from '@/components/Viewport'
import { useCurrentLibrary } from '@/lib/library'
import { Button } from '@muse/ui/v2/button'
import { Form } from '@muse/ui/v2/form'
import { useCallback, useEffect, useState } from 'react'
import { toast } from 'sonner'
import { ModelList } from './_components/ModelList'

const LibrarySettings: React.FC = () => {
  const currentLibrary = useCurrentLibrary()
  const [title, setTitle] = useState('')
  const [isPending, setIsPending] = useState(false)

  const onSubmit = useCallback(
    async (e: React.FormEvent<HTMLFormElement>) => {
      e.preventDefault()
      setIsPending(true)
      try {
        await currentLibrary.updateLibrarySettings({ title })
        toast.success('Library settings updated')
      } catch (error) {
        console.error(error)
      }
      setIsPending(false)
      // setTimeout(() => {
      //   window.location.reload()
      // }, 500)
    },
    [currentLibrary, title],
  )

  useEffect(() => {
    setTitle(currentLibrary.librarySettings.title)
  }, [currentLibrary.librarySettings.title])

  return (
    <div>
      <div className="mb-8 text-xl font-medium">Library Settings</div>
      <Form.Root onSubmit={onSubmit} className="mb-8">
        <Form.Field name="title" className="flex items-center justify-start gap-3">
          <Form.Label>Title</Form.Label>
          <Form.Input size="md" value={title} onChange={(e) => setTitle(e.currentTarget.value)} />
          <Button type="submit" variant="accent" disabled={isPending}>
            Save
          </Button>
        </Form.Field>
      </Form.Root>
    </div>
  )
}

export default function Settings() {
  return (
    <Viewport.Page>
      <Viewport.Toolbar>
        <PageNav title="Settings" />
      </Viewport.Toolbar>
      <Viewport.Content className="p-6">
        <div className="h-10">User / login / logout</div>
        <div className="bg-app-line my-4 h-px"></div>
        <LibrarySettings />
        <div className="bg-app-line my-4 h-px"></div>
        <div className="h-10">Model Settings</div>
        <ModelList />
      </Viewport.Content>
    </Viewport.Page>
  )
}
