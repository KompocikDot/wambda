import * as v from 'valibot'
import { isValidPassword } from './auth'

export const UpdateAccountSchema = v.object({
  fullName: v.pipe(
    v.string(),
    v.minLength(3, 'Full name is required'),
    v.maxLength(255, 'Full name too long'),
  ),
  email: v.pipe(v.string(), v.email('Invalid email address')),
})

export type UpdateAccountSubmitInput = v.InferInput<typeof UpdateAccountSchema>
export type UpdateAccountSubmitOutput = v.InferOutput<typeof UpdateAccountSchema>

export const ChangePasswordSchema = v.object({
  currentPassword: v.pipe(
    v.string(),
    v.check(
      isValidPassword,
      'Password should contain at least one lowercase and uppercase character, one number and one special character',
    ),
  ),
  newPassword: v.pipe(
    v.string(),
    v.check(
      isValidPassword,
      'Password should contain at least one lowercase and uppercase character, one number and one special character',
    ),
  ),
})

export type ChangePasswordSubmitInput = v.InferInput<typeof ChangePasswordSchema>
export type ChangePasswordSubmitOutput = v.InferOutput<typeof ChangePasswordSchema>
