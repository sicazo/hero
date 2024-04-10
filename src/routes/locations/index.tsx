import { createFileRoute } from '@tanstack/react-router'

export const Route = createFileRoute('/locations/')({
  component: () => <div>Hello /locations/!</div>
})