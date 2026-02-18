import { Cue } from "@/bindings";
import { TableCell, TableRow } from "@/components/ui/table";
import { cn } from "@/lib/utils";
import { useShow } from "@/state/show";
import { FC, useCallback, } from "react";

export type CueRendererProps = {
  cue: Cue,

  isCurrent: boolean,
  isSelected: boolean

  onSelect?: () => void;
};

const CueRenderer: FC<CueRendererProps> = ({ cue, onSelect, isSelected, isCurrent }) => {
  const show = useShow();
  const getActorName = useCallback((actorId: number) => {
    return show.mixConfig.actors[actorId]?.name ?? null;
  }, [show]);

  return <TableRow onClick={() => onSelect?.()} className={cn(isSelected && "bg-green-500 hover:bg-green-600", isCurrent && "bg-red-500 hover:bg-red-500", isCurrent && isSelected && "outline outline-green-500")}>
    <TableCell>{cue.id.major}.{cue.id.minor}</TableCell>
    <TableCell>{cue.name}</TableCell>
    {cue.dca.assignment.map((assignment) => (
      <TableCell className="h-10 border-r border-l">
        {assignment !== null ? (getActorName(assignment) ?? "?") : "unassigned"}
      </TableCell>
    ))}
  </TableRow>

};

export default CueRenderer;
