import { FormControl } from "@angular/forms";

type FCS = FormControl<string | null>;
export function extract_values(
  name: FCS,
  is_male: FCS,
): [string[], boolean] | undefined {
  const names = name.value?.split(",");
  name.setValue("");
  if (!names || names[0] === "") {
    return undefined;
  }
  const ismale = Boolean(is_male.value);
  is_male.setValue("1");

  return [names, ismale];
}

export function url(path: string): string {
  return `http://localhost:8080/${path}`;
}
