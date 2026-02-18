import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { useShow, useShowState } from "@/state/show";
import CueRenderer from "./CueRenderer";
import { useState } from "react";
import useEventListener from "@/hooks/domEvent";
import { mod } from "@/utils/math";
import { Button } from "@/components/ui/button";
import { Plus } from "lucide-react";
import { commands } from "@/bindings";

const NUM_DCAS = 16;

const PerformPage = () => {
  const show = useShow();
  const showState = useShowState();

  const cues = show.cues;
  const currentCueIdx = cues.findIndex(
    (cue) => cue.id === showState.currentCueId,
  );

  const [selectedCue, setSelectedCue] = useState<number | null>(null);

  useEventListener(window, "keydown", (e) => {
    switch (e.key) {
      case "ArrowDown": {
        e.preventDefault();

        const newSelected =
          selectedCue === null ? 0 : mod(selectedCue + 1, cues.length);
        setSelectedCue(newSelected);
        break;
      }
      case "ArrowUp": {
        e.preventDefault();

        const newSelected =
          selectedCue === null ? 0 : mod(selectedCue - 1, cues.length);
        setSelectedCue(newSelected);
        break;
      }
      case " ": {
        console.log("next cue");
        break;
      }
      default:
        console.log("hit key", e.key);
        break;
    }
  });

  const addCue = async () => {
    console.log("adding show");
    await commands.addCue();
  };

  return (
    <div className="size-full overflow-hidden flex flex-col p-2 gap-4">
      <div className="w-full overflow-auto grow border *:size-full">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead>Cue Id</TableHead>
              <TableHead className="w-[100px]">Cue Name</TableHead>

              {[...Array(NUM_DCAS).keys()].map((dca) => (
                <TableHead>DCA {dca}</TableHead>
              ))}
            </TableRow>
          </TableHeader>
          <TableBody>
            {cues.map((cue, index) => (
              <CueRenderer
                cue={cue}
                isCurrent={index === currentCueIdx}
                isSelected={index === selectedCue}
                onSelect={() => setSelectedCue(index)}
              />
            ))}

            <TableRow>
              <TableCell>
                <Button onClick={addCue}>
                  <Plus />
                </Button>
              </TableCell>
            </TableRow>
          </TableBody>
        </Table>
      </div>

      <div className="w-full h-20 items-end border">Hello</div>
    </div>
  );
};

export default PerformPage;
