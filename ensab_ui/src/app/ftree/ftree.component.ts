import { Component, OnInit } from '@angular/core';
import { ActivatedRoute } from '@angular/router';
import Member from './member-node/member';
import { MemberNodeComponent } from '../ftree/member-node/member-node.component';
import { HttpClientModule,HttpClient } from '@angular/common/http';
import { NgIf } from '@angular/common';

@Component({
  selector: 'app-ftree',
  standalone: true,
  imports: [MemberNodeComponent,HttpClientModule,NgIf],
  templateUrl: './ftree.component.html',
})
export class FtreeComponent implements OnInit{
  member : Member | undefined;
  create = true;//true means to create. false to update 

  show_save_button(): boolean {
    return this.create || Member.updates().length > 0 
  }

  constructor(
    private route: ActivatedRoute,
    private http : HttpClient
  ) {}

  createUpdate() {
    const member = Member.getInstance().raw();
    console.log("clicked")
    if (this.create) {
      this.http.post("http://localhost:8080/member",member).subscribe()
    } else if(Member.updates().length > 0) {
      this.http.put("http://localhost:8080/member",Member.get_and_clear_updates()).subscribe()
      console.log("puted")
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
