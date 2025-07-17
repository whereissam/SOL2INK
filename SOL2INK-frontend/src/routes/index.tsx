import { createFileRoute } from '@tanstack/react-router'
import { MigrationAssistant } from '@/components/MigrationAssistant'

export const Route = createFileRoute('/')({
  component: () => <MigrationAssistant />,
})