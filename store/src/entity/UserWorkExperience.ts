import {
  Entity,
  PrimaryGeneratedColumn,
  Column,
  CreateDateColumn,
  UpdateDateColumn,
  JoinColumn,
  OneToOne
} from "typeorm";
import { User } from "./User";

@Entity()
export class UserWorkExperience {
  @PrimaryGeneratedColumn()
  id: number;

  @OneToOne(type => User)
  @JoinColumn()
  user: User;

  @Column()
  title: string;

  @Column()
  organization: string;

  @Column()
  location: string;

  @Column()
  fromDate: string;

  @Column()
  toDate: string;

  @Column()
  description: string;

  @CreateDateColumn()
  createdAt: Date;

  @UpdateDateColumn()
  updatedAt: Date;
}