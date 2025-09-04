import * as React from "react"
import { cva, type VariantProps } from "class-variance-authority"

import { cn } from "@/lib/utils"
import {AlertCircle} from "lucide-react";

const alertVariants = cva(
  "relative w-full rounded-lg border px-4 py-3 text-sm grid has-[>svg]:grid-cols-[calc(var(--spacing)*4)_1fr] grid-cols-[0_1fr] has-[>svg]:gap-x-3 gap-y-0.5 items-start [&>svg]:size-4 [&>svg]:translate-y-0.5 [&>svg]:text-current",
  {
    variants: {
      variant: {
        default: "bg-card text-card-foreground",
        destructive:
          "text-destructive bg-card [&>svg]:text-current *:data-[slot=alert-description]:text-destructive/90",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  }
)

function Alert({
  className,
  variant,
  ...props
}: React.ComponentProps<"div"> & VariantProps<typeof alertVariants>) {
  return (
    <div
      data-slot="alert"
      role="alert"
      className={cn(alertVariants({ variant }), className)}
      {...props}
    />
  )
}

function AlertTitle({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="alert-title"
      className={cn(
        "col-start-2 line-clamp-1 min-h-4 font-medium tracking-tight",
        className
      )}
      {...props}
    />
  )
}

function AlertDescription({
  className,
  ...props
}: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="alert-description"
      className={cn(
        "text-muted-foreground col-start-2 grid justify-items-start gap-1 text-sm [&_p]:leading-relaxed",
        className
      )}
      {...props}
    />
  )
}

interface GlobalErrorProps {
  error: { global: string | null | undefined } | null
}

function GlobalError({error}: GlobalErrorProps) {
  return (
    <>
      {typeof error?.global === "string" ? (
        <Alert className={"mb-5"} variant="destructive">
          <AlertCircle/>
          <AlertDescription>
            {error.global}
          </AlertDescription>
        </Alert>
      ) : null}
    </>
  )
}

interface FieldErrorProps {
  field: string
  error: { fields: Record<string, string> | null | undefined } | null
}

function FieldError({error, field}: FieldErrorProps) {
  return (
    <>
      {typeof error?.fields === "object"
      && error.fields !== null
      && field in error.fields
      && error.fields[field] !== null ? (
        <Alert className={"mb-7"} variant="destructive">
          <AlertCircle/>
          <AlertDescription>
            {error.fields[field]}
          </AlertDescription>
        </Alert>
      ) : null}
    </>
  )
}

export {
  Alert,
  AlertTitle,
  AlertDescription,
  GlobalError,
  FieldError
}
