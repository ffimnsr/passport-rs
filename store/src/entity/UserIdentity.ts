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
export class UserIdentity {
  @PrimaryGeneratedColumn()
  id: number;

  @OneToOne(type => User)
  @JoinColumn()
  user: User;

  @Column()
  accountEmail: string;

  @Column()
  accountId: string;

  @Column()
  provider: string;

  @Column()
  publicProfileUrl: string;

  @CreateDateColumn()
  createdAt: Date;

  @UpdateDateColumn()
  updatedAt: Date;
}