import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import Member from '../member';
import { MemberNodeComponent } from '../ftree/member-node/member-node.component';

@Component({
  selector: 'app-ftree',
  standalone: true,
  imports: [MemberNodeComponent],
  templateUrl: './ftree.component.html',
})
export class FtreeComponent implements OnInit{
  member : Member = new Member("");
  constructor(private route: ActivatedRoute) { }

  ngOnInit() {
    this.route.params
      .subscribe((params) => {
        this.member = new Member(params['name'])
      }
    );
  }
}
