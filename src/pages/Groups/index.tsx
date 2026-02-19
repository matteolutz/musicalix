import { TypographyH2 } from "@/components/typography";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Dialog, DialogContent, DialogTrigger } from "@/components/ui/dialog";
import { useShow } from "@/state/show";
import { ALL_WING_COLORS, getWingColor } from "@/utils/color";
import { MicVocal } from "lucide-react";
import { z } from "zod";
import { Controller, useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import { Field, FieldGroup, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { useState } from "react";
import { commands } from "@/bindings";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import {
  Combobox,
  ComboboxChip,
  ComboboxChips,
  ComboboxChipsInput,
  ComboboxContent,
  ComboboxEmpty,
  ComboboxItem,
  ComboboxList,
  ComboboxValue,
} from "@/components/ui/combobox";

const addGroupFormSchema = z.object({
  actors: z.array(z.number().int().min(0)),
  color: z.enum(ALL_WING_COLORS).optional(),
  name: z.string(),
});

const GroupsPage = () => {
  const show = useShow();

  const actors = show.mixConfig.actors;
  const groups = show.mixConfig.groups;

  const [addGroupFormDialogOpen, setAddGroupForgDialogOpen] = useState(false);

  const addGroupForm = useForm({
    resolver: zodResolver(addGroupFormSchema),
  });

  const onAddGroupSubmit = async ({
    actors,
    name,
    color,
  }: z.infer<typeof addGroupFormSchema>) => {
    setAddGroupForgDialogOpen(false);
    addGroupForm.reset();
    await commands.addGroup(actors, name, color ?? null);
  };

  return (
    <div className="size-full overflow-y-auto flex flex-col p-2 gap-4">
      <TypographyH2>Groups</TypographyH2>

      {Object.entries(groups)
        .sort(([idA], [idB]) => idA.localeCompare(idB))
        .map(([, group]) => (
          <Card>
            <CardContent className="flex flex-row items-center gap-4">
              <Avatar>
                <AvatarFallback
                  style={{
                    backgroundColor: group!.color
                      ? getWingColor(group!.color)
                      : undefined,
                  }}
                ></AvatarFallback>
              </Avatar>
              <div className="size-full flex flex-col">
                <div className="text-lg">{group!.name}</div>
                <div className="size-full flex flex-row">
                  <div className="flex items-center gap-1 text-muted-foreground">
                    <MicVocal size="14" />{" "}
                    {group!.actors
                      .map((actorId) => actors[actorId]!.name)
                      .join(", ")}
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>
        ))}

      <Dialog
        modal
        open={addGroupFormDialogOpen}
        onOpenChange={setAddGroupForgDialogOpen}
      >
        <DialogTrigger asChild>
          <Button>Add Group</Button>
        </DialogTrigger>
        <DialogContent>
          <form
            className="flex flex-col gap-6"
            onSubmit={addGroupForm.handleSubmit(onAddGroupSubmit)}
          >
            <FieldGroup>
              <Controller
                control={addGroupForm.control}
                name="actors"
                render={({ field, fieldState }) => (
                  <Field data-invalid={fieldState.invalid}>
                    <FieldLabel htmlFor="add-actor-form-channel">
                      Actors
                    </FieldLabel>
                    <Combobox
                      multiple
                      autoHighlight
                      value={field.value ?? []}
                      onValueChange={(value) => {
                        field.onChange(
                          value.map((id) => parseInt(id as unknown as string)),
                        );
                      }}
                      items={Object.entries(actors)
                        .sort(([idA], [idB]) => idA.localeCompare(idB))
                        .map(([id]) => id)
                        .filter((id) => {
                          return !field.value?.includes(parseInt(id));
                        })}
                      id="add-actor-form-channel"
                    >
                      <ComboboxChips>
                        <ComboboxValue>
                          {field.value?.map((id) => (
                            <ComboboxChip key={id}>
                              {actors[id]!.name}
                            </ComboboxChip>
                          ))}
                        </ComboboxValue>
                        <ComboboxChipsInput placeholder="Add actor" />
                      </ComboboxChips>
                      <ComboboxContent>
                        <ComboboxEmpty>No items found.</ComboboxEmpty>
                        <ComboboxList>
                          {(id) => (
                            <ComboboxItem key={id} value={id}>
                              {actors[id]!.name}
                            </ComboboxItem>
                          )}
                        </ComboboxList>
                      </ComboboxContent>
                    </Combobox>
                  </Field>
                )}
              />
            </FieldGroup>

            <FieldGroup>
              <Controller
                control={addGroupForm.control}
                name="name"
                render={({ field, fieldState }) => (
                  <Field data-invalid={fieldState.invalid}>
                    <FieldLabel htmlFor="add-group-form-name">Name</FieldLabel>
                    <Input
                      {...field}
                      aria-invalid={fieldState.invalid}
                      id="add-group-form-name"
                      type="text"
                    />
                  </Field>
                )}
              />
            </FieldGroup>

            <FieldGroup>
              <Controller
                control={addGroupForm.control}
                name="color"
                render={({ field, fieldState }) => (
                  <Field data-invalid={fieldState.invalid}>
                    <FieldLabel htmlFor="add-group-form-color">
                      Channel
                    </FieldLabel>
                    <Select
                      name={field.name}
                      value={field.value}
                      onValueChange={field.onChange}
                    >
                      <SelectTrigger
                        id="add-group-form-color"
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

            <Button type="submit">Add</Button>
          </form>
        </DialogContent>
      </Dialog>
    </div>
  );
};

export default GroupsPage;
