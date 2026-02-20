import { commands, Cue, SingleDcaAssignment, WingColor } from "@/bindings";
import {
  Combobox,
  ComboboxCollection,
  ComboboxContent,
  ComboboxEmpty,
  ComboboxGroup,
  ComboboxInput,
  ComboboxItem,
  ComboboxLabel,
  ComboboxList,
  ComboboxSeparator,
} from "@/components/ui/combobox";
import { useShow } from "@/state/show";
import { FC, useCallback, useState } from "react";

export type CueDcaAssignmentProps = {
  cue: Cue;
  dcaIndex: number;
  allowEditing: boolean;
};

type DcaAssignmentComboboxGroupName = "actors" | "groups";

type DcaAssignmentComboboxGroup = {
  value: DcaAssignmentComboboxGroupName;
  items: { id: number; name: string; color: WingColor | null }[];
}[];

type DcaAssignmentComboboxValue = {
  group: DcaAssignmentComboboxGroupName;
  id: number;
};

const UNASSIGNED = "Unassigned";

const CueDcaAssignment: FC<CueDcaAssignmentProps> = ({
  cue,
  dcaIndex,
  allowEditing,
}) => {
  const show = useShow();
  const assignment = cue.dca.assignment[dcaIndex];

  const getInitialValue = (): DcaAssignmentComboboxValue | undefined => {
    if (assignment === "None") return undefined;
    if ("Actor" in assignment) return { group: "actors", id: assignment.Actor };
    if ("Group" in assignment) return { group: "groups", id: assignment.Group };
  };

  const [value, setValue] = useState(getInitialValue());

  const comboboxValues: DcaAssignmentComboboxGroup = [
    {
      value: "actors",
      items: Object.entries(show.mixConfig.actors)
        .sort(([idA], [idB]) => idA.localeCompare(idB))
        .map(([id, actor]) => ({
          id: parseInt(id),
          color: actor!.color,
          name: actor!.name,
        })),
    },
    {
      value: "groups",
      items: Object.entries(show.mixConfig.groups)
        .sort(([idA], [idB]) => idA.localeCompare(idB))
        .map(([id, group]) => ({
          id: parseInt(id),
          color: group!.color,
          name: group!.name,
        })),
    },
  ];

  const getAssignmentName = useCallback(
    (value: DcaAssignmentComboboxValue) => {
      if (typeof value.group === "undefined") return "?";

      const collection = show.mixConfig[value.group];
      const name = collection[value.id]?.name;

      return `${name ?? "?"} (${value.group})`;
    },
    [show],
  );

  const onValueChange = (value: DcaAssignmentComboboxValue | null) => {
    let assignment: SingleDcaAssignment = "None";
    if (value !== null) {
      switch (value.group) {
        case "actors":
          assignment = { Actor: value.id };
          break;
        case "groups":
          assignment = { Group: value.id };
          break;
      }
    }

    commands.setCueDcaAssignment(cue.id, dcaIndex, assignment).then((res) => {
      if (res.status === "ok") {
        setValue(value ?? undefined);
      }
    });
  };

  if (!allowEditing) {
    return (
      <div className="min-w-30">
        {value ? getAssignmentName(value) : UNASSIGNED}
      </div>
    );
  }

  return (
    <Combobox
      value={value}
      onValueChange={onValueChange}
      items={comboboxValues}
      itemToStringLabel={(value) => getAssignmentName(value) ?? "?"}
    >
      <ComboboxInput className="min-w-30" placeholder={UNASSIGNED} />

      <ComboboxContent className="min-w-40">
        <ComboboxEmpty>No actors or groups found</ComboboxEmpty>
        <ComboboxList>
          {(group: DcaAssignmentComboboxGroup[number], index) => (
            <ComboboxGroup key={group.value} items={group.items}>
              <ComboboxLabel>{group.value}</ComboboxLabel>
              <ComboboxCollection>
                {(
                  item: DcaAssignmentComboboxGroup[number]["items"][number],
                ) => (
                  <ComboboxItem
                    key={item.id}
                    value={{ group: group.value, id: item.id }}
                  >
                    {item.name}
                  </ComboboxItem>
                )}
              </ComboboxCollection>
              {index < comboboxValues.length - 1 && <ComboboxSeparator />}
            </ComboboxGroup>
          )}
        </ComboboxList>
      </ComboboxContent>
    </Combobox>
  );
};

export default CueDcaAssignment;
