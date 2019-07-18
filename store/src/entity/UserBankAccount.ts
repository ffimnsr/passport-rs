import {Entity, PrimaryGeneratedColumn, Column, JoinColumn, OneToOne} from "typeorm";
import {User} from "./User";

@Entity()
export class UserBankAccount {

  @PrimaryGeneratedColumn()
  id: number;

  @OneToOne(type => User)
  @JoinColumn()
  user: User;

  @Column()
  accountName: string;
  
  @Column()
  accountNo: string;

  @Column()
  bankAddress: string;

  @Column()
  bankBranch: string;

  @Column()
  swiftCode: string;

  @Column()
  createdAt: Date;

  @Column()
  updatedAt: Date;  
}

