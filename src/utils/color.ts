import { WingColor } from "@/bindings";

export const ALL_WING_COLORS = [
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

export const getWingColor = (wingColor: WingColor): string => {
  switch (wingColor) {
    case "GrayBlue":
      return "#9FE0EA";
    case "MediumBlue":
      return "#0BF1FB";
    case "DarkBlue":
      return "#28B6E8";
    case "Turquoise":
      return "#00FBEE";
    case "Green":
      return "#00EE4D";
    case "OliveGreen":
      return "#C6DC52";
    case "Yellow":
      return "#FFEF3D";
    case "Orange":
      return "#FF8D3D";
    case "Red":
      return "#FF3933";
    case "Coral":
      return "#FF916B";
    case "Pink":
      return "#FDA5F6";
    case "Mauve":
      return "#9F92FA";
  }
};
