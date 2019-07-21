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
export class ProjectIssue {
  @PrimaryGeneratedColumn()
  id: number;

  @OneToOne(type => Project)
  @JoinColumn()
  project: Project;

  @OneToOne(type => User)
  @JoinColumn()
  assignedTo: User;

  @OneToOne(type => User)
  @JoinColumn()
  reportedBy: User;

  @Column()
  code: string;

  @Column("text")
  description: string;
  
  @Column("date")
  assignedAt: Date;

  @Column("date")
  reportedAt: Date;

  @Column()
  priority: number;

  @Column()
  status: number;
  
  @CreateDateColumn()
  createdAt: Date;

  @UpdateDateColumn()
  updatedAt: Date;
}
