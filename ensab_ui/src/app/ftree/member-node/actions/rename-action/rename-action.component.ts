import { Component, input, OnInit } from "@angular/core";
import { FormControl, ReactiveFormsModule } from "@angular/forms";
import Member from "../../member";
import { ActionComponent } from "../action/action.component";
import { extract_values } from "../../../../shared";

@Component({
  selector: "rename",
  standalone: true,
  imports: [ReactiveFormsModule, ActionComponent],
  templateUrl: "./rename-action.component.html",
})
export class RenameActionComponent implements OnInit {
  member = input.required<Member>();
  name_control = new FormControl("");
  is_male = new FormControl("1");

  ngOnInit(): void {
    this.is_male.setValue(this.member().is_male() ? "1" : "");
  }

  on_submit() {
    const values = extract_values(this.name_control, this.is_male);
    if (!values) return;
    const [name, is_male] = values;
    this.member().name.set(name[0]);
    this.member().is_male.set(is_male);
    this.member().redrawAction();
  }
}
