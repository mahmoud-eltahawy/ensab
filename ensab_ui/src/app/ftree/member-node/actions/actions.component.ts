import { Component, computed, input } from "@angular/core";
import { ActionsButtonsFormComponent } from "./actions-buttons-form/actions-buttons-form.component";
import { AddSonActionComponent } from "./add-son-action/add-son-action.component";
import { RemoveSonActionComponent } from "./remove-son-action/remove-son-action.component";
import { RenameActionComponent } from "./rename-action/rename-action.component";
import Member from "../member";

@Component({
  selector: "actions",
  standalone: true,
  imports: [
    ActionsButtonsFormComponent,
    AddSonActionComponent,
    RemoveSonActionComponent,
    RenameActionComponent,
  ],
  templateUrl: "./actions.component.html",
})
export class ActionsComponent {
  member = input.required<Member>();
  check = computed(() => this.member().checkAction());
}
