import { Component, EventEmitter, input, output } from "@angular/core";

@Component({
  selector: "action-button",
  standalone: true,
  imports: [],
  templateUrl: "./action-button.component.html",
})
export class ActionButtonComponent {
  value = input.required<string>()
  click = output<EventEmitter<undefined>>()

  on_click(){
    this.click.emit(new EventEmitter())
  }
}