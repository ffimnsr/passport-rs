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
export class WithdrawalRequest {
  @PrimaryGeneratedColumn()
  id: number;

  @OneToOne(type => User)
  @JoinColumn()
  requestedById: User;

  @OneToOne(type => User)
  @JoinColumn()
  approvedById: User;

  @Column()
  bankAccountNo: number;

  @Column()
  amount: number;

  @Column()
  reference: string;

  @Column()
  approvedAt: Date;

  @CreateDateColumn()
  createdAt: Date;

  @UpdateDateColumn()
  updatedAt: Date;
}
