import {
  Entity,
  PrimaryGeneratedColumn,
  Column,
  CreateDateColumn,
  UpdateDateColumn,
  JoinColumn,
  OneToOne
} from "typeorm";
import { Project } from "./Project";
import { User } from "./User";

@Entity()
export class ProjectMember {
  @PrimaryGeneratedColumn()
  id: number;

  @OneToOne(type => Project)
  @JoinColumn()
  project: Project;

  @OneToOne(type => User)
  @JoinColumn()
  talent: User;
  
  @Column("date")
  startDate: Date;

  @Column("date")
  endDate: Date;
  
  @CreateDateColumn()
  createdAt: Date;

  @UpdateDateColumn()
  updatedAt: Date;
}
