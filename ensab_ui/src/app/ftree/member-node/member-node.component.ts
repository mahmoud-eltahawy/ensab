import { Component, computed, model } from '@angular/core';
import Member from './member';
import { NgFor, NgIf } from '@angular/common';
import { ActionsComponent } from './actions/actions.component';

@Component({
  selector: 'member-node',
  standalone: true,
  imports: [
    NgIf,
    NgFor,
    ActionsComponent
  ],
  templateUrl: './member-node.component.html',
})
export class MemberNodeComponent {
  member = model<Member>();
  actions = computed(() => this.member()?.actions)
}
