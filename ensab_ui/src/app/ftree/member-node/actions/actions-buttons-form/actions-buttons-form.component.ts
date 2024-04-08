import { Component, input } from "@angular/core";
import Member from "../../member";
import { ActionButtonComponent } from "./action-button-component";


@Component({
  selector: "actions-buttons",
  standalone: true,
  imports: [ActionButtonComponent],
  templateUrl: "./actions-buttons-form.component.html",
})
export class ActionsButtonsFormComponent {
  member = input.required<Member>();
}
