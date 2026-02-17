import { TypographyH2 } from "@/components/typography";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { useShow } from "@/state/show";
import { getWingColor } from "@/utils/color";
import { MicVocal } from "lucide-react";
import { z } from "zod";
import { Controller, useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Field, FieldGroup, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { useDebounce } from "@/hooks/debounce";
import { useEffect, useState } from "react";
import { commands } from "@/bindings";
import { numericString } from "@/utils/zod";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Spinner } from "@/components/ui/spinner";

const ALL_COLORS = [
  "GrayBlue",
  "MediumBlue",
  "DarkBlue",
  "Turquoise",
  "Green",
  "OliveGreen",
  "Yellow",
  "Orange",
  "Red",
  "Coral",
  "Pink",
  "Mauve",
] as const;

const getInitials = (name: string, maxInitials: number) =>
  name
    .split(" ")
    .map((word) => word[0])
    .slice(0, maxInitials)
    .join("");

const addActorFormSchema = z.object({
  channel: numericString(z.number().int().min(1).max(48)),
  color: z.enum(ALL_COLORS).optional(),
  name: z.string(),
});

const ActorsPage = () => {
  const show = useShow();
  const actors = show.mixConfig.actors;

  const [addActorFormDialogOpen, setAddActorFormDialogOpen] = useState(false);

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
    setAddActorFormDialogOpen(false);
    addActorForm.reset();
    await commands.addActor(channel, name, color ?? null);
  };

  return (
    <div className="size-full overflow-y-auto flex flex-col p-2 gap-4">
      <TypographyH2>Actors</TypographyH2>

      {Object.entries(actors)
        .sort(([idA], [idB]) => idA.localeCompare(idB))
        .map(([, actor]) => (
          <Card>
            <CardContent className="flex flex-row items-center gap-4">
              <Avatar>
                <AvatarFallback
                  style={{
                    backgroundColor: actor!.color
                      ? getWingColor(actor!.color)
                      : undefined,
                  }}
                >
                  {getInitials(actor!.name, 2)}
                </AvatarFallback>
              </Avatar>
              <div className="size-full flex flex-col">
                <div className="text-lg">{actor!.name}</div>
                <div className="size-full flex flex-row">
                  <div className="flex items-center gap-1 text-muted-foreground">
                    <MicVocal size="14" /> {actor!.channel}
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        ))}

      <Dialog
        open={addActorFormDialogOpen}
        onOpenChange={setAddActorFormDialogOpen}
      >
        <DialogTrigger asChild>
          <Button>Add Actor</Button>
        </DialogTrigger>
        <DialogContent>
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
                    <FieldLabel htmlFor="add-actor-form-channel">
                      Channel
                    </FieldLabel>
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
                      <FieldLabel htmlFor="add-actor-form-name">
                        Name
                      </FieldLabel>
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
                      <FieldLabel htmlFor="add-actor-form-color">
                        Channel
                      </FieldLabel>
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
                          {ALL_COLORS.map((color) => (
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
        </DialogContent>
      </Dialog>
    </div>
  );
};

export default ActorsPage;
