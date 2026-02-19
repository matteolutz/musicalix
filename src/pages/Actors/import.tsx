import { Field, FieldGroup, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";

import { Controller, useForm } from "react-hook-form";
import { numericString } from "@/utils/zod";

import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { FC } from "react";
import { commands } from "@/bindings";

import { Button } from "@/components/ui/button";

export type ImportActorsFormProps = {
  onSubmit?: () => void;
};

const importActorsFormSchema = z.object({
  fromChannel: numericString(z.number().int().min(1).max(48)),
  toChannel: numericString(z.number().int().min(1).max(48)),
});

const ImportActorsForm: FC<ImportActorsFormProps> = ({ onSubmit }) => {
  const importActorsForm = useForm({
    resolver: zodResolver(importActorsFormSchema),
  });

  const onImportActorsSubmit = async ({
    fromChannel,
    toChannel,
  }: z.infer<typeof importActorsFormSchema>) => {
    onSubmit?.();
    importActorsForm.reset();
    await commands.importActors(fromChannel, toChannel);
  };

  return (
    <form
      className="flex flex-col gap-6"
      onSubmit={importActorsForm.handleSubmit(onImportActorsSubmit)}
    >
      <FieldGroup>
        <Controller
          control={importActorsForm.control}
          name="fromChannel"
          render={({ field, fieldState }) => (
            <Field data-invalid={fieldState.invalid}>
              <FieldLabel htmlFor="add-actor-form-from-channel">
                From Channel
              </FieldLabel>
              <Input
                {...field}
                aria-invalid={fieldState.invalid}
                id="add-actor-form-from-channel"
                type="number"
              />
            </Field>
          )}
        />
      </FieldGroup>

      <FieldGroup>
        <Controller
          control={importActorsForm.control}
          name="toChannel"
          render={({ field, fieldState }) => (
            <Field data-invalid={fieldState.invalid}>
              <FieldLabel htmlFor="add-actor-form-to-channel">
                To Channel
              </FieldLabel>
              <Input
                {...field}
                aria-invalid={fieldState.invalid}
                id="add-actor-form-to-channel"
                type="number"
              />
            </Field>
          )}
        />
      </FieldGroup>

      <Button type="submit">Add</Button>
    </form>
  );
};

export default ImportActorsForm;
