import { commands, Cue } from "@/bindings";
import { Input } from "@/components/ui/input";
import { TableCell, TableRow } from "@/components/ui/table";
import { cn } from "@/lib/utils";
import { FC, useCallback, useEffect, useState } from "react";
import CueDcaAssignment from "./assignment";
import { Button } from "@/components/ui/button";
import { Trash } from "lucide-react";
import { useConfirmationModalContext } from "@/hooks/modal";

export type CueRendererProps = {
  cue: Cue;

  isCurrent: boolean;
  isSelected: boolean;
  allowEditing: boolean;

  onSelect?: () => void;
};

const CueRenderer: FC<CueRendererProps> = ({
  cue,
  onSelect,
  isSelected,
  isCurrent,
  allowEditing,
}) => {
  const [cueName, setCueName] = useState(cue.name);

  useEffect(() => {
    setCueName(cue.name);
  }, [cue]);

  const submitCueName = () => {
    commands.renameCue(cue.id, cueName).then((res) => {
      if (res.status !== "ok") {
        setCueName(cue.name);
      }
    });
  };

  const cm = useConfirmationModalContext();

  const deleteCue = useCallback(() => {
    cm.showConfirmation({
      title: `Delete Cue ${cue.id.major}.${cue.id.minor}`,
      message:
        "Do you really want to delete this cue? This action cannot be undone!",
      confirmButtonText: "Delete",
      confirmButtonVariant: "destructive",
    }).then((res) => {
      if (res) {
        commands.deleteCue(cue.id);
      }
    });
  }, [cue]);

  return (
    <TableRow
      onClick={() => onSelect?.()}
      className={cn(
        "duration-20",
        isSelected && "bg-green-500 hover:bg-green-600",
        isCurrent && "bg-red-500 hover:bg-red-500",
        isCurrent && isSelected && "outline outline-green-500",
      )}
    >
      <TableCell>
        {allowEditing && (
          <Button onClick={deleteCue} variant="ghost" size="icon">
            <Trash size={8} />
          </Button>
        )}
      </TableCell>
      <TableCell>
        {cue.id.major}.{cue.id.minor}
      </TableCell>
      <TableCell>
        <div className="min-w-40">
          {allowEditing ? (
            <Input
              className="m-0"
              onKeyDown={(e) => e.key === "Enter" && e.currentTarget.blur()}
              onBlur={submitCueName}
              onChange={(e) => setCueName(e.target.value)}
              value={cueName}
            />
          ) : (
            cue.name
          )}
        </div>
      </TableCell>
      {cue.dca.assignment.map((_, idx) => (
        <TableCell className="h-10 border-r border-l">
          <CueDcaAssignment
            cue={cue}
            dcaIndex={idx}
            allowEditing={allowEditing}
          />
        </TableCell>
      ))}
    </TableRow>
  );
};

export default CueRenderer;
