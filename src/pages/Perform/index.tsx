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
import { useCallback, useEffect, useState } from "react";
import useEventListener from "@/hooks/domEvent";
import { mod } from "@/utils/math";
import { Button } from "@/components/ui/button";
import { Plus } from "lucide-react";
import { commands } from "@/bindings";
import { Switch } from "@/components/ui/switch";
import { useConfirmationModalContext } from "@/hooks/modal";

const NUM_DCAS = 16;

const PerformPage = () => {
  const show = useShow();
  const showState = useShowState();

  const cues = show.cues;
  const currentCueIdx = cues.findIndex(
    (cue) => cue.id === showState.currentCueId,
  );

  const [isPerforming, setIsPerforming] = useState<boolean>(false);
  const [selectedCueIdx, setSelectedCueIdx] = useState<number | null>(null);
  const selectedCue = selectedCueIdx !== null ? cues[selectedCueIdx] : null;

  const cm = useConfirmationModalContext();

  useEffect(() => {
    if (!isPerforming) {
      // reset selected cue when arming performance mode
      setSelectedCueIdx(cues.length > 0 ? 0 : null);
    } else {
      setSelectedCueIdx(null);
    }
  }, [cues, isPerforming]);

  const addToCue = useCallback(
    (add: number) => {
      const newSelected =
        selectedCueIdx === null ? 0 : mod(selectedCueIdx + add, cues.length);
      setSelectedCueIdx(newSelected);
    },
    [selectedCueIdx],
  );

  useEventListener(
    window,
    "keydown",
    (e) => {
      switch (e.key) {
        case "ArrowDown": {
          e.preventDefault();

          addToCue(1);
          break;
        }
        case "ArrowUp": {
          e.preventDefault();

          addToCue(-1);
          break;
        }
        case " ": {
          if (!isPerforming || selectedCueIdx === null) break;

          const cueId = cues[selectedCueIdx].id;
          commands.gotoCue(cueId).then((res) => {
            if (res.status === "ok") {
              addToCue(1);
            }
          });
          break;
        }
        default:
          console.log("hit key", e.key);
          break;
      }
    },
    [selectedCueIdx],
  );

  const addCue = async () => {
    console.log("adding show");
    await commands.addCue();
  };

  const onPerformingChanged = (isPerforming: boolean) => {
    if (isPerforming) {
      setIsPerforming(true);
      return;
    }

    cm.showConfirmation({
      title: "Disarm performance mode",
      message: "Do you really want to disarm the performance mode?",
      confirmButtonText: "Disarm",
    }).then((res) => {
      if (res) {
        setIsPerforming(false);
      }
    });
  };

  return (
    <div className="size-full overflow-hidden flex flex-col p-2 gap-4">
      <div className="w-full p-2 gap-2 h-10 flex items-center border">
        <p className="text-sm">Arm perform</p>
        <Switch checked={isPerforming} onCheckedChange={onPerformingChanged} />
      </div>

      <div className="w-full overflow-auto grow border *:size-full">
        <Table>
          <TableHeader>
            <TableRow>
              <TableHead />
              <TableHead>Cue Id</TableHead>
              <TableHead className="w-25">Cue Name</TableHead>

              {[...Array(NUM_DCAS).keys()].map((dca) => (
                <TableHead>DCA {dca}</TableHead>
              ))}
            </TableRow>
          </TableHeader>
          <TableBody>
            {cues.map((cue, index) => (
              <CueRenderer
                cue={cue}
                isCurrent={isPerforming && index === currentCueIdx}
                isSelected={isPerforming && index === selectedCueIdx}
                allowEditing={!isPerforming}
                onSelect={() => setSelectedCueIdx(index)}
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

      <div className="w-full h-20 items-end border">
        {isPerforming && selectedCue && selectedCue.name}
      </div>
    </div>
  );
};

export default PerformPage;
