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

@Entity()
export class ProjectClue {
  @PrimaryGeneratedColumn()
  id: number;

  @OneToOne(type => Project)
  @JoinColumn()
  project: Project;

  @Column("text")
  description: string;

  @Column()
  repoHttpUrl: string;

  @Column()
  repoSshUrl: string;

  @Column()
  repoWebUrl: string;
  
  @CreateDateColumn()
  createdAt: Date;

  @UpdateDateColumn()
  updatedAt: Date;
}
