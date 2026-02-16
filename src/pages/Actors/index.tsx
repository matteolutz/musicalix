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
import { Field, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { useDebounce } from "@/hooks/debounce";
import { useEffect, useState } from "react";
import { commands } from "@/bindings";
import { numericString } from "@/utils/zod";

const getInitials = (name: string, maxInitials: number) =>
  name
    .split(" ")
    .map((word) => word[0])
    .slice(0, maxInitials)
    .join("");

const addActorFormSchema = z.object({
  channel: numericString(z.number().int().min(1).max(48)),
  color: z.enum([
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
  ]),
  name: z.string(),
});

const ActorsPage = () => {
  const show = useShow();
  const actors = show.mixConfig.actors;

  const addActorForm = useForm({
    resolver: zodResolver(addActorFormSchema),
  });

  const [_isFetchingChannelInfo, setIsFetchingChannelInfo] = useState(false);

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

  const onAddActorSubmit = (_data: z.infer<typeof addActorFormSchema>) => {};

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

      <Dialog>
        <DialogTrigger asChild>
          <Button>Add Actor</Button>
        </DialogTrigger>
        <DialogContent>
          <form onSubmit={addActorForm.handleSubmit(onAddActorSubmit)}>
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
          </form>
        </DialogContent>
      </Dialog>
    </div>
  );
};

export default ActorsPage;
