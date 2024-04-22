import { Component, OnInit } from "@angular/core";
import { ActivatedRoute } from "@angular/router";
import Member, { RawMember } from "./member-node/member";
import { MemberNodeComponent } from "../ftree/member-node/member-node.component";
import { HttpClient, HttpClientModule } from "@angular/common/http";

@Component({
  selector: "app-ftree",
  standalone: true,
  imports: [MemberNodeComponent, HttpClientModule],
  templateUrl: "./ftree.component.html",
})
export class FtreeComponent implements OnInit {
  member: Member | undefined;

  constructor(
    private route: ActivatedRoute,
    private http: HttpClient,
  ) {}

  saveUpdates() {
    Member.updates.commit(this.http);
  }

  discardUpdates() {
    Member.updates.discard();
  }

  ngOnInit() {
    this.route.params
      .subscribe((params) => {
        this.member = Member.getInstance(params["name"]);
      });
    this.route.queryParams.subscribe((params) => {
      const id: string | undefined = params["id"];
      if (id) {
        try {
          this.http.get(`http://localhost:8080/member/${id}`)
            .subscribe((x) => {
              const member = Member.getInstanceFromRaw(x as RawMember);
              this.member = member;
            });
        } catch (e) {
          console.log(e);
        }
      }
    });
  }
}
