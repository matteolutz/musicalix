import { TypographyH2 } from "@/components/typography";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { Button } from "@/components/ui/button";
import { Card, CardContent } from "@/components/ui/card";
import { Dialog, DialogContent } from "@/components/ui/dialog";
import { useShow } from "@/state/show";
import { getWingColor } from "@/utils/color";
import { MicVocal } from "lucide-react";

import { useState } from "react";
import AddActorForm from "./add";
import ImportActorsForm from "./import";

const getInitials = (name: string, maxInitials: number) =>
  name
    .split(" ")
    .map((word) => word[0])
    .slice(0, maxInitials)
    .join("");

const ActorsPage = () => {
  const show = useShow();
  const actors = show.mixConfig.actors;

  const [addActorFormDialogOpen, setAddActorFormDialogOpen] = useState(false);
  const [importActorsFormDialogOpen, setImportActorsFormDialogOpen] =
    useState(false);

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

      <div className="size-full flex gap-2 p-2">
        <Button onClick={() => setAddActorFormDialogOpen(true)}>
          Add Actor
        </Button>

        <Button onClick={() => setImportActorsFormDialogOpen(true)}>
          Import Actors
        </Button>
      </div>

      <Dialog
        open={addActorFormDialogOpen}
        onOpenChange={setAddActorFormDialogOpen}
        modal
      >
        <DialogContent>
          <AddActorForm onSubmit={() => setAddActorFormDialogOpen(false)} />
        </DialogContent>
      </Dialog>

      <Dialog
        open={importActorsFormDialogOpen}
        onOpenChange={setImportActorsFormDialogOpen}
        modal
      >
        <DialogContent>
          <ImportActorsForm
            onSubmit={() => setImportActorsFormDialogOpen(false)}
          />
        </DialogContent>
      </Dialog>
    </div>
  );
};

export default ActorsPage;
