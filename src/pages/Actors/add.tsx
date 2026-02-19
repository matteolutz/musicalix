import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Spinner } from "@/components/ui/spinner";
import { Field, FieldGroup, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";

import { Controller, useForm } from "react-hook-form";
import { numericString } from "@/utils/zod";

import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod";
import { FC, useEffect, useState } from "react";
import { commands } from "@/bindings";
import { useDebounce } from "@/hooks/debounce";
import { ALL_WING_COLORS, getWingColor } from "@/utils/color";
import { Button } from "@/components/ui/button";

export type AddActorFormProps = {
  onSubmit?: () => void;
};

const addActorFormSchema = z.object({
  channel: numericString(z.number().int().min(1).max(48)),
  color: z.enum(ALL_WING_COLORS).optional(),
  name: z.string(),
});

const AddActorForm: FC<AddActorFormProps> = ({ onSubmit }) => {
  const addActorForm = useForm({
    resolver: zodResolver(addActorFormSchema),
  });

  const [isFetchingChannelInfo, setIsFetchingChannelInfo] = useState(false);

  const fetchChannelInfo = async (channelId: number) => {
    console.log("fetching", channelId);

    const response = await commands.getWingChannelInfo(channelId);
    if (response.status !== "ok") {
      console.error("Failed to fetch channel info:", response.error);
      return;
    }

    const channelInfo = response.data;
    addActorForm.setValue("color", channelInfo.color);
    addActorForm.setValue("name", channelInfo.name);
  };

  const channelValue = addActorForm.watch("channel");
  const debouncedChannel = useDebounce(channelValue, 200);
  useEffect(() => {
    if (typeof debouncedChannel === "undefined") {
      return;
    }

    setIsFetchingChannelInfo(true);
    fetchChannelInfo(parseInt(debouncedChannel as unknown as string)).then(() =>
      setIsFetchingChannelInfo(false),
    );
  }, [debouncedChannel]);

  const onAddActorSubmit = async ({
    channel,
    name,
    color,
  }: z.infer<typeof addActorFormSchema>) => {
    onSubmit?.();
    addActorForm.reset();
    await commands.addActor(channel, name, color ?? null);
  };

  return (
    <form
      className="flex flex-col gap-6"
      onSubmit={addActorForm.handleSubmit(onAddActorSubmit)}
    >
      <FieldGroup>
        <Controller
          control={addActorForm.control}
          name="channel"
          render={({ field, fieldState }) => (
            <Field data-invalid={fieldState.invalid}>
              <FieldLabel htmlFor="add-actor-form-channel">Channel</FieldLabel>
              <Input
                {...field}
                aria-invalid={fieldState.invalid}
                id="add-actor-form-channel"
                type="number"
              />
            </Field>
          )}
        />
      </FieldGroup>

      <div className="flex flex-col gap-6 relative">
        <FieldGroup>
          <Controller
            control={addActorForm.control}
            disabled={isFetchingChannelInfo}
            name="name"
            render={({ field, fieldState }) => (
              <Field data-invalid={fieldState.invalid}>
                <FieldLabel htmlFor="add-actor-form-name">Name</FieldLabel>
                <Input
                  {...field}
                  aria-invalid={fieldState.invalid}
                  id="add-actor-form-name"
                  type="text"
                />
              </Field>
            )}
          />
        </FieldGroup>

        <FieldGroup>
          <Controller
            control={addActorForm.control}
            disabled={isFetchingChannelInfo}
            name="color"
            render={({ field, fieldState }) => (
              <Field data-invalid={fieldState.invalid}>
                <FieldLabel htmlFor="add-actor-form-color">Channel</FieldLabel>
                <Select
                  name={field.name}
                  value={field.value}
                  onValueChange={field.onChange}
                >
                  <SelectTrigger
                    id="add-actor-form-color"
                    aria-invalid={fieldState.invalid}
                  >
                    <SelectValue placeholder="Choose a color" />
                  </SelectTrigger>
                  <SelectContent>
                    {ALL_WING_COLORS.map((color) => (
                      <SelectItem value={color} className="flex gap-2">
                        <span
                          className="w-5 h-5 rounded-full"
                          style={{ backgroundColor: getWingColor(color) }}
                        />
                        {color}
                      </SelectItem>
                    ))}
                  </SelectContent>
                </Select>
              </Field>
            )}
          />
        </FieldGroup>

        {isFetchingChannelInfo && (
          <div className="size-full absolute bg-black/50 flex justify-center items-center">
            <Spinner />
          </div>
        )}
      </div>

      <Button type="submit">Add</Button>
    </form>
  );
};

export default AddActorForm;
