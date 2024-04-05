import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import Member from './member-node/member';
import { MemberNodeComponent } from '../ftree/member-node/member-node.component';
import { HttpClientModule,HttpClient } from '@angular/common/http';

@Component({
  selector: 'app-ftree',
  standalone: true,
  imports: [MemberNodeComponent,HttpClientModule],
  templateUrl: './ftree.component.html',
})
export class FtreeComponent implements OnInit{
  member : Member | undefined;
  create = true;//true means to create. false to update 
  constructor(
    private route: ActivatedRoute,
    private http : HttpClient
  ) {}

  createUpdate() {
    if (this.create) {
      this.http.post("http://localhost:8080/member",Member.getInstance().raw())
        .subscribe(x => {
        console.log(x)
      })
      console.log("create")
    } else {
      console.log("update")
    }
  }

  ngOnInit() {
    this.route.params
      .subscribe(params => {
        this.member = Member.getInstance(params['name'])
      }
    );
    this.route.queryParams.subscribe(params => {
       const id : string | undefined= params['id']; 
       if(id) {
        try {
          this.http.get(`http://localhost:8080/member/${id}`)
          .subscribe(x => {
            const member = Member.getInstanceFromRaw(x as any);
            this.member = member;
            this.create = false;
          });
        } catch(e) {
          console.log(e) 
        }
       }
    })
  }
}
