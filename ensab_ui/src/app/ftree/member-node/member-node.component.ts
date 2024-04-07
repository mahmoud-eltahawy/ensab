import { Component, computed, model } from '@angular/core';
import Member from './member';
import { ActionsComponent } from './actions/actions.component';

@Component({
  selector: 'member-node',
  standalone: true,
  imports: [
    ActionsComponent
  ],
  templateUrl: './member-node.component.html',
})
export class MemberNodeComponent {
  member = model<Member>();
  actions = computed(() => this.member()?.actions)
}
