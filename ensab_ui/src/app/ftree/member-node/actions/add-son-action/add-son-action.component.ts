import { Component, input, signal } from "@angular/core";
import { FormControl, ReactiveFormsModule } from "@angular/forms";
import Member from "../../member";
import { ActionComponent } from "../action/action.component";
import { extract_values } from "../../../../shared";

@Component({
  selector: "add-son",
  standalone: true,
  imports: [ReactiveFormsModule, ActionComponent],
  templateUrl: "./add-son-action.component.html",
})
export class AddSonActionComponent {
  name = new FormControl("");
  is_male = new FormControl("1");
  member = input.required<Member>();

  on_submit() {
    const values = extract_values(this.name, this.is_male);
    if (!values) return;
    const [names, is_male] = values;
    for (const name of names) {
      const member = Member.create_from_name(name); 
      member.is_male.set(is_male);
      this.member().add_son(member);
    }
    this.member().redrawAdd();
  }

  is_only = signal(true);

  on_input() {
    const value = this.name.value;
    if (value?.includes(",")) {
      this.is_only.set(false);
    } else {
      this.is_only.set(true);
    }
  }
}
